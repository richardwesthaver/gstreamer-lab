extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::env;

fn main_loop(uri: &str) -> Result<(), glib::BoolError> {
    ges::init()?;

    // Begin by creating a timeline with audio and video tracks
    let timeline = ges::Timeline::new_audio_video();
    // Create a new layer that will contain our timed clips.
    let layer = timeline.append_layer();
    let pipeline = ges::Pipeline::new();
    pipeline.set_timeline(&timeline)?;

    // Load a clip from the given uri and add it to the layer.
    let clip = ges::UriClip::new(uri).expect("Failed to create clip");
    layer.add_clip(&clip)?;

    // Add an effect to the clip's video stream.
    let effect = ges::Effect::new("agingtv").expect("Failed to create effect");
    clip.add(&effect).unwrap();

    println!(
        "Agingtv scratch-lines: {}",
        clip.get_child_property("scratch-lines")
            .unwrap()
            .serialize()
            .unwrap()
    );

    // Retrieve the asset that was automatically used behind the scenes, to
    // extract the clip from.
    let asset = clip.get_asset().unwrap();
    let duration = asset
        .downcast::<ges::UriClipAsset>()
        .unwrap()
        .get_duration();
    println!(
        "Clip duration: {} - playing file from {} for {}",
        duration,
        duration / 2,
        duration / 4
    );

    // The inpoint specifies where in the clip we start, the duration specifies
    // how much we play from that point onwards. Setting the inpoint to something else
    // than 0, or the duration something smaller than the clip's actual duration will
    // cut the clip.
    clip.set_inpoint(duration / 2);
    clip.set_duration(duration / 4);

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let bus = pipeline.get_bus().unwrap();
    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    Ok(())
}

#[allow(unused_variables)] // not necessarily needed
fn main() {
    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: ges launch");
        std::process::exit(-1)
    };

    match main_loop(uri) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}

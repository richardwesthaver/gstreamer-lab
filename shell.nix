{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    glib
    glib-networking
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
    gst_all_1.gst-plugins-good
    gst_all_1.gst-plugins-bad
    gst_all_1.gst-plugins-ugly
    gst_all_1.gst-editing-services
    gdk-pixbuf
    gtk3
    cairo
    pango
    atk
    carnix
    pkg-config
  ];
  shellHook = ''
  export GIO_EXTRA_MODULES="${pkgs.glib-networking.out}/lib/gio/modules";
  '';
}

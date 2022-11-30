# Bevy 6315 - Premultiplied alpha demo

This repo is relevant to [Bevy issue #6315](https://github.com/bevyengine/bevy/issues/6315). It demonstrates undoing alpha that has been applied in gamma-corrected space and producing an image with premultiplied alpha applied in linear space.

It expects an image called `spineboy-eye-pma.png` to be placed in the root directory of this project and produces the corrected image as `spineboy-eye-corrected-pma.png`.

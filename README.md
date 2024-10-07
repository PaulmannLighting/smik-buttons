# smik-buttons

Handle events of the smik gateway's buttons.

## Usage

The smik gateway currently has one hardware button, which serves two purposes:

1. Reset the gateway to factory settings.
2. Upload a log dump to the smik cloud.

In order to performs these two actions with one button and to prevent accidental factory resets, the button must be
pressed within certain constraints:

1. To reset the gateway to factory settings, the button must be pressed five times within ten seconds.
2. To upload a log dump to the smik cloud, the button must be pressed and held for five to twenty seconds.

## Examples

The folder `examples` contains a few examples on how to use the library.

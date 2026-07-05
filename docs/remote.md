# Remote

Snout supports a remote control interface allowing you to control various features of the application remotely.
This uses OSC (Open Sound Control). A binary is provided (`snout-remote`) that can be used.

## Configuration

Add the following to your `config.toml` file:

```toml
[control]
listen = "127.0.0.1:9500"
```

## `snout-remote` usage

### Setting face bounds

To set the bounds of a face shape, run the following command:
```sh
snout-remote face-bounds <shape> <lower> <upper>
```


### Face auto calibration

You can auto calibrate the lower bounds of the face shapes using the `face-calibrate` command.
This will take a 100 frames worth of data (about 3 seconds at 30fps) and use it to determine the lower bounds of the face shapes.

Make sure to keep a neutral face through the calibration cycle.

You can add the `-v` flag to `snout-cli` to see the set bounds.
Bounds are *not* saved persistently.

```sh
snout-remote face-calibrate
```

### Face calibrate upper bound

You can calibrate the upper bound of a face shape using the `face-calibrate-upper` command.
Optionally you can override the amount of frames used for calibration, using the `--frames <n>` flag.

```sh
snout-remote face-calibrate-upper <shape> [--frames <n>]
```

This will capture N frames (default 100) and use them to determine the upper bound of the face shape.
Try and keep the maximum of the particular shape you're trying to calibrate.

## API

### Set face bounds

Set the bounds of the face.

```
/snout/face/bounds <shape> <lower> <upper>
```

### Start face auto calibration

Start the auto calibration process for the face.

```
/snout/face/calibrate
```


#### Start face upper calibration

Start the upper calibration process for the face.

```
/snout/face/calibrate/upper <shape> <frames>
```

## Available face shapes

- cheekPuffLeft
- cheekPuffRight
- cheekSuckLeft
- cheekSuckRight
- jawOpen
- jawForward
- jawLeft
- jawRight
- noseSneerLeft
- noseSneerRight
- mouthFunnel
- mouthPucker
- mouthLeft
- mouthRight
- mouthRollUpper
- mouthRollLower
- mouthShrugUpper
- mouthShrugLower
- mouthClose
- mouthSmileLeft
- mouthSmileRight
- mouthFrownLeft
- mouthFrownRight
- mouthDimpleLeft
- mouthDimpleRight
- mouthUpperUpLeft
- mouthUpperUpRight
- mouthLowerDownLeft
- mouthLowerDownRight
- mouthPressLeft
- mouthPressRight
- mouthStretchLeft
- mouthStretchRight
- tongueOut
- tongueUp
- tongueDown
- tongueLeft
- tongueRight
- tongueRoll
- tongueBendDown
- tongueCurlUp
- tongueSquish
- tongueFlat
- tongueTwistLeft
- tongueTwistRight

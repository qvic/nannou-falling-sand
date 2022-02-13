# Falling sand game
![App preview](https://user-images.githubusercontent.com/8373424/153779250-0a88f248-4b44-4f1c-aa6b-4a6903eefd54.png)

Created with [Nannou](https://github.com/nannou-org/nannou).

### Usage

Left click to spawn cells, right click to enable eraser. Switch materials with keyboard. The defaults are:

- **Sand** - Key 1
- **Water** - Key 2
- **Wall** - Key 3
- **Plague** - Key 4
- **Acidic sand** - Key 5

### Game materials

Materials are not hardcoded. You can edit JSON configuration in `materials.json` - add your own materials or modify
existing.

For example, this way you can define red sand that moves down, if bottom cell is free, and replicates to the left or
right, if there's someone:

```json
{
  "id": 5,
  "name": "Red sand",
  "color": "FF0000",
  "key": "Key6",
  "rules": [
    {
      "movement": {"Move": {"row": 1, "column": 0}},
      "if_empty": [{"row": 1, "column": 0}],
      "if_occupied": []
    },
    {
      "movement": {"Copy": {"row": 0, "column": -1}},
      "if_empty": [],
      "if_occupied": [{"row": 0, "column": -1}]
    },
    {
      "movement": {"Copy": {"row": 0, "column": 1}},
      "if_empty": [],
      "if_occupied": [{"row": 0, "column": 1}]
    }
  ]
}
```

Algorithm searches for first matching rule, if any, and updates the cell.

### Build from source

```shell
cargo run --release
```

If graphics glitches on Wayland, use:

```shell
WINIT_UNIX_BACKEND=x11 cargo run --release
```

# DIF

Device Information File

A library for turning ``.dif`` files into a Rust struct. Made for [Novusk](https://github.com/new-kerne/novusk).

DIFs are used for giving bare metal apps or kernels some information about the device and some of the things that need 
to be done. ``.dif`` files are similar to ``.json`` files which makes them easy to write, [here](todo) is some 
documentation on how to write them.

[Library documentaion](todo)

---

``example.dif``:
```json
[
  ("DifFileName", "example.dif"),
]
```

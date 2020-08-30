# Getting started

This is a project to teach you how image compression algorithm wokrs.

But currently it only supports the feature to convert the uploaded jpg file to a grayscale file.


## Build and run

Notes, in order to successfully build and run with the project, you should prepare the enviornment as instructed (here)[https://www.secondstate.io/articles/setup-rust-nodejs/].

```
# install dependencies
npm i
# build
npm run build
# start the server at http://localhost:8080
npm run start
```

## Notes

According to the issue https://github.com/image-rs/image/issues/879, we have to disable the "jpeg_rayon" feature in Cargo.toml.

To save converted image in server, we should use wasi + wasm https://www.secondstate.io/articles/wasi-access-system-resources/.


## Resources

This project is inspired by [this post](http://pi.math.cornell.edu/~web6140/TopTenAlgorithms/JPEG.html) and [this post](https://inst.eecs.berkeley.edu/~ee123/sp16/Sections/JPEG_DCT_Demo.html) and this fantastic [video](https://youtu.be/Ba89cI9eIg8). You can also learn more about image compression algorithm from them.


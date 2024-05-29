# ORM Generator

This is a simple tool to generate an [ORM Map](https://forum.reallusion.com/457280/What-does-the-ORM-Map-Do) from three other input textures.

- The **Red** channel of the first input texture (occlusion file) is **AO** value.
- The **Green** channel of the second input texture (roughness file) is **Roughness** value.
- The **Blue** Channel of the third input texture (metallic file) is **Metallic** value.

> Note: the three input textures should have same size.

And the generated ORM map has three color channels:
- **Red** channel: **AO**
- **Green** channel: **Roughness**
- **Blue** channel: **Metallic**

## Run

```shell
cargo run
```

## Usage
Run the execute and you will see the user interface or window like the snapshoot below:
![alt text](./res/example_0.png)


For example, you choose the three input textures as below:
- Occlusion file:
![occlusion input](./res/occlusion.jpg)
- Roughness file:
![roughness input](./res/roughness.jpg)
- Metallic file:
![metallic input](./res/metallic.jpg)

And click the **Generate ORM Map** button:
![click Generate ORM Map](./res/example_1.png)

You will get an image (ORM Map) like this one:
![The generated ORM Map](./res/orm.jpg)

You can see the path of the generated file at **"Done. The generated ORM map: xxx"**.

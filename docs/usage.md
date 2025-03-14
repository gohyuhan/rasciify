#  🖥️ Usage
- [Image to Text](#image-to-text)
- [Image to Grayscale Image](#image-to-grayscale-image)
- [Image to RGB Image](#image-to-rgb-image)
- [GIF to Grayscale GIF](#gif-to-grayscale-gif)
- [GIF to RGB GIF](#gif-to-rgb-gif)

# 🔠 CharacterType
To see all the supported characters list, check out [here](character.md)

# ⚙️ SettingOption
| parameter        | type             | description                                                                                |
|------------------|------------------|--------------------------------------------------------------------------------------------|
| num_cols         | `u32`            | Number of columns of the generated ASCII art                                               | 
| is_white_bg      | `bool`           | Decide the background color of the ASCII art, default as black                             |
| is_color         | `bool`           | Decide if the ASCII art should be RGB or grayscale                                         |

&nbsp;
## Image to Text
To generate ASCII art in the form of text from an image, you can use the `image_to_text()` method. This method will save the ASCII art in the form of a txt file.
`image_to_text()` takes in 5 parameter in the following sequence.
| parameter        | type             | description                                                                                |
|------------------|------------------|--------------------------------------------------------------------------------------------|
| path             | `&str`           | Path of the image file                                                                     |    
| num_cols         | `u32`            | Number of columns of the generated ASCII art                                               |  
| complex          | `bool`           | If should use complex or simple symbols                                                    |
| output_directory | `Option<&str>`   | Path of the directory where the ASCII art should be saved                                  |
| filename         | `Option<&str>`   | Name for the ASCII art file ( extension was not required)                                  |

Example:
```rust
use rasciify::img_to_text::image_to_text;

let _ = image_to_text(
    "test.jpg",
    200,
    false,
    None,
    Some("test_ascii_txt"),
);
```
Example Output:
![Example Output](/examples/text-example.png)


### Result as String
---
To get the result back as `String` for further modification, you can use the `grayscale_to_ascii()` method, which is the same method used within `image_to_text()` for the core process of converting an image to ASCII art in text base.
`grayscale_to_ascii` takes in 3 parameter in the following sequence.
| parameter        | type             | description                                                                                |
|------------------|------------------|--------------------------------------------------------------------------------------------|
| img              | `&DynamicImage`  | The image                                                                                  |    
| num_cols         | `u32`            | Number of columns of the generated ASCII art                                               |  
| complex          | `bool`           | If should use complex or simple symbols                                                    |

Example:
```rust
use rasciify::img_to_text::grayscale_to_ascii;

let img = image::open("< Image Path >").expect("Failed to open image");
let ascii_string:String = grayscale_to_ascii(
    &img,
    200,
    false,
);
```

&nbsp;
## Image to Grayscale Image
To generate ASCII art in the form of grayscale image from an image, you can use the `image_to_image()` method. This method will save the ASCII art in the form of jpg image file.
`image_to_image()` takes in 5 parameter in the following sequence.
| parameter        | type             | description                                                                                |
|------------------|------------------|--------------------------------------------------------------------------------------------|
| path             | `&str`           | Path of the image file                                                                     |     
| character        | `CharacterType`  | The characters to be used for the ASCII art                                                |
| output_directory | `Option<&str>`   | Path of the directory where the ASCII art should be saved                                  |
| filename         | `Option<&str>`   | Name for the ASCII art file ( extension was not required)                                  |
| setting_option   | `SettingOption`  | Additional settings for the ASCII art generation                                           |

Example:
```rust
use rasciify::{
    img_to_text::image_to_image,
    character::CharacterType
    types::SettingOption
};

let _ = image_to_image(
        "test.jpg",
        CharacterType::JpHiragana,
        None,
        Some("test_ascii_grayscale_img"),
        SettingOption::grayscale(200)
    );
```
Example Output:
![Example Output](/examples/test_ascii_grayscale_img.jpg)


### Result as ImageBuffer<Luma<u8>, Vec<u8>>
---
To get the result back as `ImageBuffer<Luma<u8>, Vec<u8>>` for further modification, you can use the `grayscale_to_ascii_img()` method, which is the same method used within `image_to_image()` for the core process of converting an image to ASCII art in grayscale image form.
`grayscale_to_ascii_img` takes in 3 parameter in the following sequence.
| parameter        | type             | description                                                                                |
|------------------|------------------|--------------------------------------------------------------------------------------------|
| img              | `&DynamicImage`  | The image                                                                                  |   
| character        | `CharacterType`  | The characters to be used for the ASCII art                                                |
| setting_option   | `SettingOption`  | Additional settings for the ASCII art generation                                           |

Example:
```rust
use rasciify::{
    img_to_text::grayscale_to_ascii_img,
    character::CharacterType,
    types::SettingOption
};

let img = image::open("< Image Path >").expect("Failed to open image");
let ascii_grayscale_image_buffer:ImageBuffer<Luma<u8>, Vec<u8>> = grayscale_to_ascii(
    &img,
    CharacterType::JpHiragana,
    SettingOption::grayscale(200)
);
```

&nbsp;
## Image to RGB Image
The same as [Image to Grayscale Image](#image-to-grayscale-image) where parameter `is_color` of `setting_option` is set to true, you can call `SettingOption::rgb(<num_cols>)`. This method will save the ASCII art in the form of png image file.

Example:
```rust
use rasciify::{
    img_to_text::image_to_image,
    character::CharacterType,
    types::SettingOption
};

let _ = image_to_image(
        "test.jpg",
        CharacterType::JpHiragana,
        None,
        Some("test_ascii_rgb_img"),
        SettingOption::rgb(200)
    );
```
Example Output:
![Example Output](/examples/test_ascii_rgb_img.jpeg)


### Result as ImageBuffer<Rgba<u8>, Vec<u8>>
---
To get the result back as `ImageBuffer<Rgba<u8>, Vec<u8>>` for further modification, you can use the `rgb_to_rgb_ascii_img()` method, which is the same method used within `image_to_image()` for the core process of converting an image to ASCII art in rgb image form.
`rgb_to_rgb_ascii_img` takes in 3 parameter in the following sequence.
| parameter        | type             | description                                                                                |
|------------------|------------------|--------------------------------------------------------------------------------------------|
| img              | `&DynamicImage`  | The image                                                                                  |
| character        | `CharacterType`  | The characters to be used for the ASCII art                                                |
| setting_option   | `SettingOption`  | Additional settings for the ASCII art generation                                           |

Example:
```rust
use rasciify::{
    img_to_text::rgb_to_rgb_ascii_img,
    character::CharacterType,
    types::SettingOption
};

let img = image::open("< Image Path >").expect("Failed to open image");
let ascii_rgb_image_buffer:ImageBuffer<Rgba<u8>, Vec<u8>> = rgb_to_rgb_ascii_img(
    &img,
    CharacterType::JpHiragana,
    SettingOption::rgb(200)
);
```

&nbsp;
## GIF to Grayscale GIF
To generate ASCII gif in the form of grayscale gif from a gif, you can use the `gif_to_gif()` method. This method will save the grayscale ASCII gif in the form of gif file.

> [!WARNING] 
> The dimension of each frame in the gif need to be consistent, else unpredictable behaviour might occur

`gif_to_gif()` takes in 5 parameter in the following sequence.
| parameter        | type             | description                                                                                |
|------------------|------------------|--------------------------------------------------------------------------------------------|
| path             | `&str`           | Path of the gif file                                                                       | 
| character        | `CharacterType`  | The characters to be used for the ASCII gif                                                |
| output_directory | `Option<&str>`   | Path of the directory where the ASCII gif should be saved                                  |
| filename         | `Option<&str>`   | Name for the ASCII gif file ( extension was not required)                                  |
| setting_option   | `SettingOption`  | Additional settings for the ASCII art generation                                           |

Example:
```rust
use rasciify::{
    gif_to_gif::gif_to_gif,
    character::CharacterType,
    types::SettingOption
};

let _ = image_to_image(
        "test.gif",
        CharacterType::JpHiragana,
        None,
        Some("test_ascii_grayscale_gif"),
        SettingOption::grayscale(200)
    );
```
Example Output:
![Example Output](/examples/test_gray_gif.gif)
Original GIF by [Leroy Patterson](https://giphy.com/leroypatterson) from [giphy.com](https://giphy.com/gifs/leroypatterson-cat-glasses-CjmvTCZf2U3p09Cn0h).


&nbsp;
## GIF to RGB GIF
To generate ASCII gif in the form of RGB gif from a gif, you can use the `gif_to_gif()` method. This method will save the RGB ASCII gif in the form of gif file.

> [!WARNING] 
> The dimension of each frame in the gif need to be consistent, else unpredictable behaviour might occur

`gif_to_gif()` takes in 5 parameter in the following sequence.
| parameter        | type             | description                                                                                |
|------------------|------------------|--------------------------------------------------------------------------------------------|
| path             | `&str`           | Path of the gif file                                                                       | 
| character        | `CharacterType`  | The characters to be used for the ASCII gif                                                |
| output_directory | `Option<&str>`   | Path of the directory where the ASCII gif should be saved                                  |
| filename         | `Option<&str>`   | Name for the ASCII gif file ( extension was not required)                                  |
| setting_option   | `SettingOption`  | Additional settings for the ASCII art generation                                           |

Example:
```rust
use rasciify::{
    gif_to_gif::gif_to_gif,
    character::CharacterType,
    types::SettingOption
};

let _ = image_to_image(
        "test.gif",
        CharacterType::JpHiragana,
        None,
        Some("test_ascii_grayscale_gif"),
        SettingOption::rgb(200)
    );
```
Example Output:
![Example Output](/examples/test_rgb_gif.gif)
Original GIF by [Leroy Patterson](https://giphy.com/leroypatterson) from [giphy.com](https://giphy.com/gifs/leroypatterson-cat-glasses-CjmvTCZf2U3p09Cn0h).

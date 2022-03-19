# NOTES

- [NOTES](#notes)
  - [Links](#links)
  - [TLDR](#tldr)
  - [Setup your Yew development environment](#setup-your-yew-development-environment)
    - [Project Setup](#project-setup)
      - [Install WebAssembly target](#install-webassembly-target)
      - [Install Trunk](#install-trunk)
    - [Build a sample app](#build-a-sample-app)
      - [Create Project](#create-project)
      - [Converting the project into a Yew web application](#converting-the-project-into-a-yew-web-application)
      - [Create index.html](#create-indexhtml)
      - [View your web application](#view-your-web-application)
      - [Congratulations](#congratulations)
  - [How to set up Tailwind CSS with Yew and Trunk](#how-to-set-up-tailwind-css-with-yew-and-trunk)
    - [Links](#links-1)
    - [Installing Things](#installing-things)
    - [Launch Dev Enviroment](#launch-dev-enviroment)
  - [VSCode Debug and Settings](#vscode-debug-and-settings)
    - [Vscode Tailwind Intellisense](#vscode-tailwind-intellisense)
    - [Vscode Debug Yew](#vscode-debug-yew)

## Links

- [What is Yew? | Yew](https://yew.rs/)
- [Project Setup | Yew](https://yew.rs/docs/getting-started/introduction)

## TLDR

launch develop environment

```shell
# win#1
$ cd yew-app/
$ trunk serve

# win#2
$ cd yew-app/
$ npx tailwindcss -i ./src/input.css -o ./srcToutput.css --watch
```

## Setup your Yew development environment

> the above is almost a copy paste from oficial docs

### Project Setup

> The minimum supported Rust version (MSRV) for Yew is 1.56.0. Older versions can cause unexpected issues accompanied by incomprehensible error messages or outright fail to compile. You can check your toolchain version using `rustup show` (under "active toolchain") or alternatively `rustc --version`. To update your toolchain, run `rustup update`.

```shell
$ rustc --version
rustc 1.59.0 (9d1b2106e 2022-02-23)
$ rustup update
# same
$ rustc --version
rustc 1.59.0 (9d1b2106e 2022-02-23)
# we are good to go
```

#### Install WebAssembly target

```shell
$ rustup target add wasm32-unknown-unknown
```

#### Install Trunk

> Trunk is the recommended tool for managing deployment and packaging

```shell
$ cargo install trunk
```

### Build a sample app

#### Create Project

To get started, create a new cargo project.

```shell
$ cargo new yew-app
```

Open the newly created directory.

```shell
$ cd yew-app
```

#### Converting the project into a Yew web application

To convert this simple command line application to a basic Yew web application, a few changes are needed.

Update `Cargo.toml`

Add yew to the list of dependencies.

`Cargo.toml`

```toml
[dependencies]
# you can check the latest version here: https://crates.io/crates/yew
yew = "0.19.3"
```

Update `main.rs`

We need to generate a template which sets up a **root Component called Model** which renders a button that updates its value when clicked. Replace the contents of `src/main.rs` with the following code.
note

> The line `yew::start_app::<Model>()` inside `main()` **starts your application** and **mounts it** to the page's `<body>` tag.
> If you would like to start your application with any dynamic properties, you can instead use `yew::start_app_with_props::<Model>(..)`.

#### Create index.html

Finally, add an `index.html` file in the root directory of your app.

`index.html`

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Yew App</title>
  </head>
</html>
```

#### View your web application

Run the following command to build and serve the application locally.

```shell
$ trunk serve
```

#### Congratulations

You have now **successfully setup your Yew development environment**, and built your first web application.

## How to set up Tailwind CSS with Yew and Trunk

### Links

- [How to set up Tailwind CSS with Yew and Trunk](https://dev.to/arctic_hen7/how-to-set-up-tailwind-css-with-yew-and-trunk-il9)
- [Installation: Tailwind CLI - Tailwind CSS](https://tailwindcss.com/docs/installation)
- [Classes | Yew](https://yew.rs/docs/concepts/html/classes)

### Installing Things 

```shell
$ sudo npm i -g tailwindcss
$ tailwindcss --help
tailwindcss v3.0.23
# init
$ npx tailwindcss init
```

add to `yew-app/src/main.rs`

```rust
    html! {
			<div class={classes!("bg-red-100", "m-4")}>
				// use all classes in same line with format!
				<p class={classes!("text-red text-4xl font-bold underline uppercase".to_string())}>
					{"Tailwind Css"}
				</p>
        <button class={classes!(button_class)} onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
        <button class={classes!(button_class)} onclick={link.callback(|_| Msg::SubOne)}>{ "-1" }</button>
        <button class={classes!(button_class)} onclick={link.callback(|_| Msg::Reset)}>{ "Reset" }</button>
        <p class={classes!("bg-")}>
					{ self.value }
				</p>
      </div>
    }
```

add to `index.html`

```html
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link data-trunk href="./output.css" rel="css">
```

This directs Trunk to use the Tailwind file as a stylesheet. You can now use styles in your Yew `html!`, try it out like this:

add `yew-app/input.css`

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

```shell
$ tailwindcss -i input.css -o ./tailwind.css
# generate output.css, optinal use flag `--watch`
$ npx tailwindcss -i ./input.css -o ./output.css
```

> warn - No utility classes were detected in your source files. If this is unexpected, double-check the `content` option in your Tailwind CSS configuration.

the above is because it must detect used files, and what classes we use to include in that file in this case `main.rs`, like `bg-blue-100`, to fix we we must add our loved `rs` file extension

change content: `["./src/**/*.{html,js}"]`, to content: `["./src/**/*.{html,rs}"],` in `yew-app/tailwind.config.js`

add `"./**/*.html"` too in case we add some classes in `index.html`

`yew-app/tailwind.config.js`

```json
module.exports = {
  content: [
    "./**/*.html",
    "./src/**/*.{html,rs}"
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

```shell
# now launch again and it works
$ npx tailwindcss -i ./input.css -o ./output.css --watch
Done in 149ms.
```

now search `output.css` to check if `bg-blue-100` is included

### Launch Dev Enviroment

```shell
# win1
$ npx tailwindcss -i ./input.css -o ./output.css --watch

# win2
$ trunk serve
```

## VSCode Debug and Settings

### Vscode Tailwind Intellisense

- [Get Tailwind Intellisense Anywhere - Paolo Tiu](https://www.paolotiu.com/blog/get-tailwind-intellisense-anywhere)
- [How can I enable tailwind intelliSense outside of &quot;className&quot;?](https://stackoverflow.com/questions/66614875/how-can-i-enable-tailwind-intellisense-outside-of-classname)
- [Tailwind CSS IntelliSense and Twin · Discussion #227 · ben-rogerson/twin.macro](https://github.com/ben-rogerson/twin.macro/discussions/227)

> I know, regex is hard, so heres a repo containing multiple use cases. <https://github.com/paolotiu/tailwind-regex-list>

`.vscode/settings.json`

```json
{
  "tailwindCSS.includeLanguages": {
    "rust": "rs"
  },
  "editor.quickSuggestions": {
    "strings": true
  },
  "tailwindCSS.experimental.classRegex": [
    // classes!("")
    "classes!\\(\"([^\"\\)]*)",
    // r#""#
    "#\"([^\"#]*)",
  ],
}
```

### Vscode Debug Yew

- [What is the state of debugging? · Issue #1981 · rustwasm/wasm-bindgen](https://github.com/rustwasm/wasm-bindgen/issues/1981)

seems isn't possible yet to debug yew wasm projects......

## Adding Tauri to a Yew Project

- [Create a desktop app in Rust using Tauri and Yew](https://dev.to/stevepryde/create-a-desktop-app-in-rust-using-tauri-and-yew-2bhe)

$ cargo install tauri-cli --locked --version ^1.0.0-rc
$ cargo tauri init
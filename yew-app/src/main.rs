use yew::prelude::*;

enum Msg {
  AddOne,
  SubOne,
  Reset,
}

struct Model {
  value: i64,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self { value: 0 }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::AddOne => {
        self.value += 1;
        // the value has changed so we need to
        // re-render for it to appear on the page
        true
      }
      Msg::SubOne => {
        self.value -= 1;
        true
      }
      Msg::Reset => {
        self.value = 0;
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
    let link = ctx.link();
    // use all classes in same line with let
    let disabled;
    if self.value == 0 {
      disabled = "bg-slate-200"
    } else {
      disabled = "bg-slate-100"
    };
    let button_base_class = r#"mr-4 my-4 p-4 rounded-xl border-2 border-slate-300"#;
		let button_class = format!("{} {}", button_base_class, r#"bg-slate-100"#);
    let button_rest_class = format!("{} {}", button_base_class, disabled);

    html! {
      <div class={classes!("m-4 p-4  text-center bg-blue-100 border-blue-300 border-2 rounded-xl".to_string())}>
        // use all classes in same line with format! or .to_string()
        <p class={classes!("text-red text-4xl font-bold underline uppercase".to_string())}>
          {"Yew, Tauri, and TailwindCss Starter Project"}
        </p>
        <button class={classes!(button_class.clone())} onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
        <button class={classes!(button_class.clone())} onclick={link.callback(|_| Msg::SubOne)}>{ "-1" }</button>
        <button disabled={self.value == 0} class={classes!(button_rest_class)} onclick={link.callback(|_| Msg::Reset)}>{ "Reset" }</button>
        <p class={classes!("p-4 text-6xl bg-red-100 border-red-300 border-2 rounded-xl".to_string())}>
          { self.value }
        </p>
      </div>
    }
  }
}

fn main() {
  yew::start_app::<Model>();
}

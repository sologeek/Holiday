#![cfg(test)]
use crate::prelude::*;
use std::{cell::RefCell, rc::Rc};
#[derive(Clone, Default, Debug)]
struct EmbedKeyPost {
  title: Rc<RefCell<&'static str>>,
  author: &'static str,
  content: &'static str,
  level: usize,
}

impl CombinationWidget for EmbedKeyPost {
  fn build<'a>(&self) -> Box<dyn Widget + 'a> {
    let mut children = vec![
      KeyDetect::new(0, Text(*self.title.borrow())).into(),
      KeyDetect::new(1, Text(self.author)).into(),
      KeyDetect::new(2, Text(self.content)).into(),
    ];

    if self.level > 0 {
      let mut embed = self.clone();
      embed.level -= 1;
      children.push(KeyDetect::new("embed", embed).into())
    }
    KeyDetect::new(0, Row(children)).into()
  }
}

pub struct KeyDetectEnv<'a> {
  pub app: Application<'a>,
  pub title: Rc<RefCell<&'static str>>,
}

impl<'a> KeyDetectEnv<'a> {
  pub fn new(level: usize) -> Self {
    let mut post = EmbedKeyPost::default();
    post.level = level;
    let title = post.title.clone();
    post.title = title.clone();

    let mut env = KeyDetectEnv {
      app: Application::default(),
      title,
    };
    let Application {
      widget_tree,
      render_tree,
      ..
    } = &mut env.app;
    widget_tree.set_root(post.into(), render_tree);

    env
  }
}

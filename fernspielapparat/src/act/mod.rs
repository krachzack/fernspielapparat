mod act;
mod actuators;
mod ring;
pub mod speech;

pub use act::Act;
pub use actuators::Actuators;
pub use ring::Ring;

#[cfg(test)]
mod test {
    //use crate::act::bell::Bell;
    use crate::act::Act;
    use tavla::{any_voice, Voice};

    #[test]
    fn put_speech_in_box_and_deref() {
        let voice = any_voice().unwrap();

        let mut act: Box<dyn Act> = Box::new(voice.speak("Heyo!").unwrap());

        assert!(!act.done().unwrap());
        act.cancel().unwrap();
        assert!(act.done().unwrap());
    }

    #[test]
    fn make_act_vector() {
        let acts: Vec<Box<dyn Act>> = vec![
            //Box::new(Bell),
            Box::new(any_voice().unwrap().speak("Heyo!").unwrap()),
        ];

        for mut act in acts {
            assert!(!act.done().unwrap());
            act.cancel().unwrap();
            assert!(act.done().unwrap());
        }
    }

}

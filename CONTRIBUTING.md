# Contributing to DokuKraft

Hello Developers, Welcome to an other open source(OSS) project **DokuKraft**. I'm trying to make a more easier **Documentation** website builder with more features and nice user interface. I know it is very tough and time consuming to build. But it is open to all to contribute. I value your time. And thank you from my heart if you are contributing.

> Suggesting A feature is also a part of **Contribution**.

I/We do not have any complex rules and guidelines for the contribution. But you must follow the two things mentioned below.
- Use **Commitizen Commit Guideline**. [Read More](https://commitizen-tools.github.io/commitizen/)
- Try to explain everyline so that new programmers/developers also understand what we are doing and how it is working.

Example:
```rust
//for backing up an event in the parser's state, ensuring that only one event is backed up at a time, and logging the action for debugging purposes.
    fn back(&mut self, ev: Event<'a>) {
        //asserts that the back field of the struct (or whatever self is referring to) is currently None.
        //this assertion ensures that there is no event already stored for backing up. If there is, it will panic.
        assert!(self.back.is_none());
        //logs a trace message, indicating that an event is being backed up. It logs the backed-up event ev.
        trace!("Back: {:?}", ev);
        //sets the back field of the struct (or whatever self is referring to) to Some(ev), storing the event for backing up
        //this event can later be retrieved when needed to rewind the parser's state.
        self.back = Some(ev);
    }
```

>TIP: You can use `push.sh` to push your code to your repository. Make sure that you have customized the `push.sh` according to your operating system. 

 
 - [How to Contribute to Open Source](https://opensource.guide/how-to-contribute/)
 - [Building Welcoming Communities](https://opensource.guide/building-community/)

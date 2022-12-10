
#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Opc {
    noop,
    addx(i32)
}

impl Opc {
    fn ticks(&self) -> usize {
        match self {
            Opc::noop => 1,
            Opc::addx {..} => 2,
        }
    }

    fn exec(&self, x: &mut i32) {
        if let Opc::addx(v) = self { *x += v }
    }
}

#[derive(Debug)]
enum State {
    Running,
    Done,
}

#[derive(Debug)]
pub struct Vm {
    ip: usize,
    reg_x: i32,
    state: State,
    ticks: usize,
    program: Vec<Opc>,
}

impl Vm {
    pub fn new(input: &str) -> Self {
        let program = Vm::compile(input);
        let ticks   = program[0].ticks();

        Vm { ip: 0, reg_x: 1, state: State::Running, ticks, program }
    }

    pub fn do_tick(&mut self) {
        if let State::Running = self.state {
            self.ticks -= 1;
            if self.ticks == 0 {
                self.program[self.ip].exec(&mut self.reg_x);
                self.ip += 1;
                if self.ip == self.program.len() {
                    self.state = State::Done
                } else {
                    self.ticks = self.program[self.ip].ticks();
                }
            }
        }
    }

    pub fn getx(&self) -> i32 {
        self.reg_x
    }

    fn compile(input: &str) -> Vec<Opc> {
        input.lines()
            .map(|line| 
                if let Some((_, v)) = line.split_once(' ') {
                    Opc::addx(v.parse::<i32>().unwrap())
                } else {
                    Opc::noop
                }
            )
            .collect()
    }
}
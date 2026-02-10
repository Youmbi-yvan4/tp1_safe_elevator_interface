#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElevatorError {
    InvalidFloor(u8),
    DoorsAlreadyOpen,
    DoorsAlreadyClosed,
    CannotOpenWhileMoving,
    CannotMoveDoorsOpen,
    EmptyQueue,
}

pub struct Elevator {
    floor: u8,
    state: State,
    queue: Vec<u8>,
}

impl Elevator {
    pub fn new(start: u8) -> Result<Self, ElevatorError> {
        if start > 5 {
            return Err(ElevatorError::InvalidFloor(start));
        }

        Ok(Self {
            floor: start,
            state: State::Idle,
            queue: Vec::new(),
        })
    }

    pub fn floor(&self) -> u8 {
        self.floor
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn queue(&self) -> &[u8] {
        &self.queue
    }

    pub fn status(&self) -> Elevator {
        Elevator {
            floor: self.floor,
            state: self.state,
            queue: self.queue.clone(),
        }
    }

    pub fn call(&mut self, floor: u8) -> Result<(), ElevatorError> {
        if floor > 5 {
            return Err(ElevatorError::InvalidFloor(floor));
        }

        if floor == self.floor {
            return Ok(());
        }

        if self.queue.contains(&floor) {
            return Ok(());
        }

        self.queue.push(floor);

        if self.state == State::Idle {
            self.update_direction();
        }

        Ok(())
    }

    pub fn update_direction(&mut self) {
        if let Some(&target) = self.queue.first() {
            if target > self.floor {
                self.state = State::MovingUp;
            } else if target < self.floor {
                self.state = State::MovingDown;
            } else {
                self.state = State::DoorsOpen;
            }
        }
    }

    pub fn open_doors(&mut self) -> Result<(), ElevatorError> {
        match self.state {
            State::DoorsOpen => Err(ElevatorError::DoorsAlreadyOpen),
            State::MovingUp | State::MovingDown => Err(ElevatorError::CannotOpenWhileMoving),
            _ => {
                self.state = State::DoorsOpen;
                Ok(())
            }
        }
    }

    pub fn close_doors(&mut self) -> Result<(), ElevatorError> {
        match self.state {
            State::DoorsOpen => {
                if self.queue.is_empty() {
                    self.state = State::Idle;
                } else {
                    self.update_direction();
                }

                Ok(())
            }
            _ => Err(ElevatorError::DoorsAlreadyClosed),
        }
    }

    pub fn step(&mut self) -> Result<(), ElevatorError> {
        if self.state == State::DoorsOpen {
            return Err(ElevatorError::CannotMoveDoorsOpen);
        }

        if self.queue.is_empty() {
            self.state = State::Idle;
            return Err(ElevatorError::EmptyQueue);
        }

        let target = self.queue[0];

        if self.floor < target {
            self.floor += 1;
            self.state = State::MovingUp;
        }
         else if self.floor > target {
            self.floor -= 1;
            self.state = State::MovingDown;
        }
        if self.floor == target {
            self.queue.remove(0);
            self.state = State::DoorsOpen;
        }

        Ok(())
    }
}

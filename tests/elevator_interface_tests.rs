use tp_1_safe_elevator_interface::{Elevator, ElevatorError, State};

#[test]
fn call_and_step_reaches_destination_and_opens_doors() {
    let mut elevator = Elevator::new(0).expect("valid start floor");

    elevator.call(3).expect("call accepted");
    assert_eq!(elevator.state(), State::MovingUp);

    elevator.step().expect("step 1");
    assert_eq!(elevator.floor(), 1);
    assert_eq!(elevator.state(), State::MovingUp);

    elevator.step().expect("step 2");
    assert_eq!(elevator.floor(), 2);
    assert_eq!(elevator.state(), State::MovingUp);

    elevator.step().expect("step 3");
    assert_eq!(elevator.floor(), 3);
    assert_eq!(elevator.state(), State::DoorsOpen);
    assert!(elevator.queue().is_empty());
}

 #[test]
 fn step_with_open_doors_returns_error() {
    let mut elevator = Elevator::new(0).expect("valid start floor");

     elevator.open_doors().expect("doors open");

     let result = elevator.step();
     assert_eq!(result, Err(ElevatorError::CannotMoveDoorsOpen));
 }

 #[test]
 fn call_with_invalid_floor_returns_error() {
     let mut elevator = Elevator::new(0).expect("valid start floor");

     let result = elevator.call(9);

     assert_eq!(result, Err(ElevatorError::InvalidFloor(9)));
}

 #[test]
 fn open_doors_while_moving_returns_error() {
     let mut elevator = Elevator::new(0).expect("valid start floor");

     elevator.call(2).expect("call accepted");
     let result = elevator.open_doors();

     assert_eq!(result, Err(ElevatorError::CannotOpenWhileMoving));
 }

 #[test]
 fn close_doors_with_no_open_doors_returns_error() {
     let mut elevator = Elevator::new(0).expect("valid start floor");

     let result = elevator.close_doors();

     assert_eq!(result, Err(ElevatorError::DoorsAlreadyClosed));
 }

 #[test]
 fn status_returns_snapshot() {
     let mut elevator = Elevator::new(0).expect("valid start floor");
     elevator.call(4).expect("call accepted");

     let status = elevator.status();

     assert_eq!(status.floor, 0);
     assert_eq!(status.state, State::MovingUp);
     assert_eq!(status.queue, vec![4]);
 }

#[test]
fn duplicate_and_current_floor_calls_are_ignored() {
    let mut elevator = Elevator::new(1).expect("valid start floor");

    elevator.call(1).expect("same floor allowed");
    elevator.call(3).expect("call accepted");
    elevator.call(3).expect("duplicate ignored");

    assert_eq!(elevator.queue(), &[3]);
}

#[test]
fn close_doors_sets_direction_when_queue_exists() {
    let mut elevator = Elevator::new(2).expect("valid start floor");

    elevator.open_doors().expect("open doors from idle");
    elevator.call(5).expect("call accepted");
    elevator.close_doors().expect("close doors");

    assert_eq!(elevator.state(), State::MovingUp);
}

#[test]
fn step_with_empty_queue_returns_error() {
    let mut elevator = Elevator::new(0).expect("valid start floor");

    let result = elevator.step();

    assert_eq!(result, Err(ElevatorError::EmptyQueue));
    assert_eq!(elevator.state(), State::Idle);
}

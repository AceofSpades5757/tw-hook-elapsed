use tasklib::prelude::*;

pub fn add_elapsed(task: Task) -> Task {
    let start = task.start();
    let end = task.end();

    if start.is_none() || end.is_none() {
        return task;
    }

    let start = start.unwrap();
    let end = end.unwrap();
    let existing_elapsed = task.udas().get("elapsed").unwrap_or(&UdaValue::Duration(Duration::days(0))).clone();
    let existing_elapsed: Duration = existing_elapsed.try_into().unwrap();

    let duration = end.signed_duration_since(*start);
    let elapsed: Duration = duration.into();

    let total_elapsed = existing_elapsed + elapsed;

    // UDAs come out just like any other value.
    let mut modified_task = task.clone();
    modified_task.udas_mut().insert("elapsed".into(), total_elapsed.into());

    modified_task
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn start_and_end() {
        let input = r#"{"description":"test","entry":"20210101T000000Z","modified":"20210101T000000Z","status":"pending","tags":["test"],"uuid":"00000000000000000000000000000000",
        "start":"20210101T020000Z",
        "end":"20210101T040000Z"
    }"#;
        let expected = r#"{"description":"test","entry":"20210101T000000Z","elapsed":"P2H","modified":"20210101T000000Z","status":"pending","tags":["test"],"uuid":"00000000000000000000000000000000",
        "start":"20210101T020000Z",
        "end":"20210101T040000Z"
    }"#;

        let input_uda = add_elapsed(input.into()).udas().get("elapsed").unwrap().clone();
        let input_elapsed = match input_uda {
            UdaValue::Duration(d) => d,
            UdaValue::String(s) => s.parse::<Duration>().unwrap(),
            _ => panic!("not a duration"),
        };
        let expected_uda = expected.parse::<Task>().unwrap().udas().get("elapsed").unwrap().clone();
        let expected_elapsed = match expected_uda {
            UdaValue::Duration(d) => d,
            UdaValue::String(s) => s.parse::<Duration>().unwrap(),
            _ => panic!("not a duration"),
        };

        assert_eq!(input_elapsed, expected_elapsed);
    }
    #[test]
    fn no_start_no_end() {
        let input = r#"{"uuid":"00000000000000000000000000000000", "description":"test", "entry":"20210101T000000Z", "modified":"20210101T000000Z", "status":"pending"}"#;
        let expected = input;
        assert_eq!(
            add_elapsed(input.into()),
            expected.parse::<Task>().unwrap(),
        );
    }
    #[test]
    fn no_start() {
        let input = r#"{"uuid":"00000000000000000000000000000000", "description":"test", "entry":"20210101T000000Z", "modified":"20210101T000000Z", "status":"pending", "end":"20210101T020000Z"}"#;
        let expected = input;
        assert_eq!(
            add_elapsed(input.into()),
            expected.parse::<Task>().unwrap(),
        );
    }
    #[test]
    fn no_end() {
        let input = r#"{"uuid":"00000000000000000000000000000000", "description":"test", "entry":"20210101T000000Z", "modified":"20210101T000000Z", "status":"pending", "start":"20210101T020000Z"}"#;
        let expected = input;
        assert_eq!(
            add_elapsed(input.into()),
            expected.parse::<Task>().unwrap(),
        );
    }
    #[test]
    fn existing_elapsed_start_and_end() {
        let input = r#"{"uuid":"00000000000000000000000000000000", "description":"test", "entry":"20210101T000000Z", "modified":"20210101T000000Z", "status":"pending",
        "start":"20210101T020000Z",
        "end":"20210101T040000Z",
        "elapsed":"PT1H"
    }"#;
        let expected = r#"{"uuid":"00000000000000000000000000000000", "description":"test", "entry":"20210101T000000Z", "modified":"20210101T000000Z", "status":"pending",
        "start":"20210101T020000Z",
        "end":"20210101T040000Z",
        "elapsed":"PT3H"
    }"#;
        let input_uda = add_elapsed(input.into()).udas().get("elapsed").unwrap().clone();
        let input_elapsed = match input_uda {
            UdaValue::Duration(d) => d,
            UdaValue::String(s) => s.parse::<Duration>().unwrap(),
            _ => panic!("not a duration"),
        };
        let expected_uda = expected.parse::<Task>().unwrap().udas().get("elapsed").unwrap().clone();
        let expected_elapsed = match expected_uda {
            UdaValue::Duration(d) => d,
            UdaValue::String(s) => s.parse::<Duration>().unwrap(),
            _ => panic!("not a duration"),
        };
        assert_eq!(
            input_elapsed,
            expected_elapsed,
        );
    }
    #[test]
    fn existing_elapsed_no_start_no_end() {
        let input = r#"{"uuid":"00000000000000000000000000000000", "description":"test", "entry":"20210101T000000Z", "modified":"20210101T000000Z", "status":"pending",
        "elapsed":"PT1H"
    }"#;
        let expected = input;
        assert_eq!(
            add_elapsed(input.into()),
            expected.parse::<Task>().unwrap(),
        );
    }
    #[test]
    fn existing_elapsed_no_start() {
        let input = r#"{"uuid":"00000000000000000000000000000000", "description":"test", "entry":"20210101T000000Z", "modified":"20210101T000000Z", "status":"pending",
        "end":"20210101T040000Z",
        "elapsed":"PT1H"
    }"#;
        let expected = input;
        assert_eq!(
            add_elapsed(input.into()),
            expected.parse::<Task>().unwrap(),
        );
    }
    #[test]
    fn existing_elapsed_no_end() {
        let input = r#"{"uuid":"00000000000000000000000000000000", "description":"test", "entry":"20210101T000000Z", "modified":"20210101T000000Z", "status":"pending",
        "start":"20210101T040000Z",
        "elapsed":"PT1H"
    }"#;
        let expected = input;
        assert_eq!(
            add_elapsed(input.into()),
            expected.parse::<Task>().unwrap(),
        );
    }
}

pub struct Time;

impl Time {
    pub fn seconds(seconds: u32) -> u32 {
        return seconds * 1000;
    }

    pub fn minutes(minutes: u32) -> u32 {
        return Time::seconds(minutes) * 60;
    }

    pub fn hours(hours: u32) -> u32 {
        return Time::minutes(hours) * 60;
    }

    pub fn days(days: u32) -> u32 {
        return Time::hours(days) * 24;
    }
}

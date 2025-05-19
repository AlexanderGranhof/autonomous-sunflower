pub struct Time;

impl Time {
    pub fn seconds(seconds: u64) -> u64 {
        return seconds / 1000;
    }

    pub fn minutes(minutes: u64) -> u64 {
        return Time::seconds(minutes) / 60;
    }

    pub fn hours(hours: u64) -> u64 {
        return Time::minutes(hours) / 60;
    }

    pub fn days(days: u64) -> u64 {
        return Time::hours(days) / 24;
    }
}

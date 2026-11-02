<?php

declare(strict_types=1);

/*
 * Oracle cases for DateTimeInstantiationRule's port of timelib's date parser (crates/pun-analyse/src/rules/
 * date_time.rs): `DateTime::getLastErrors()['errors']` after `new DateTime($s)` on the system PHP, for hand-picked
 * strings, combinations of scanner tokens and seeded random strings.
 *
 *   php scripts/export-date-time-errors.php [OUT] [RANDOM_COUNT]
 *
 * OUT defaults to crates/pun-analyse/tests/fixtures/date-time/errors.json. Inputs are written byte for byte as
 * ISO-8859-1 code points, so any byte string survives JSON.
 */

$out = $argv[1] ?? dirname(__DIR__).'/crates/pun-analyse/tests/fixtures/date-time/errors.json';
$randomCount = (int) ($argv[2] ?? 1500);

$fixed = [
    '', ' ', "\t\n", "\0", "a\0b", 'now', 'NOW', 'today', 'midnight', 'noon', 'tomorrow', 'yesterday',
    '2020.11.17', '2020.11.18', 'asdfasdf', '2020-01-01', '2020-01-01 10:00', '2020-01-01 10:00:00.5',
    '2020-01-01T10:00:00Z', '2020-01-01T10:00:00.123456+02:00', '2020-01-01T10:00:00.5GMT+02:00',
    '20200101T10:00:00', '20200101t100000', '2020-1-1T1:2:3', '2020:01:01 10:00:00', '10/Oct/2000:13:55:36 -0700',
    '10/Oct/2000:13:55:36 +07:00:00', '@1126396800', '@-1', '@1.5', '@-1.5', '@1.', '@123 @456', '@123 @456 @789',
    '@99999999999999999999', '+1 week', '+1 week 2 days', '-1 day', '+1 µs', '+1 µsec', '1 fortnight ago',
    'next monday', 'last day of next month', 'first day of this month', 'first monday of january 2020',
    'last sat of July 2008', 'this week', 'next week', 'previous week', 'back of 7pm', 'front of 7', 'back of 24',
    'Jan 5', 'Jan 5th', '5 Jan', '5 January 2020', 'January 5, 2020', 'Jan-05-2020', '2020-Jan-05', 'Jan 2020',
    '2020 Jan', 'Jan 5 10:30', 'Jan 5 10.30.45', 'Jan 5 10:30:45.5', 'Jan 5 10:30:45 CET', 'Jan 5 10.30.45 CET',
    'Jan 5 10:30:45 Europe/Amsterdam', 'Jan 5 10:30:45 Foo/Bar', 'Jan 5 10:30am', 'Jan 5 10:30:45 pm', '10am',
    '10 am', '10:30 a.m.', '10.30pm', '12:00:00.5 PM', '12:00:00:5PM', '2020W01', '2020-W01-3', '2020W013',
    '2020.123', '2020-366', 'II', 'XII 2020', '1 II 2020', 'Europe/Amsterdam', 'europe/amsterdam',
    'Europe/AMSTERDAM', 'America/Argentina/ComodRivadavia', 'Foo/Bar', 'CET', 'cet', '(CET)', '(CET', 'CET)',
    'UTC', 'utc', 'GMT', 'GMT+5', 'GMT+05:00', 'GMT-0530', '+05:00', '-05', '+0530', '+053000', '+05:30:00',
    '+05:3000', 'Etc/GMT+5', 'Z', 'z', 'EST5EDT', 'MST7MDT', 'Universal', 'Iceland', 'Zulu', 'NZ-CHAT',
    '10:00 CET CEST', '10:00 CET CEST WET', 'CET 10:00 CEST 11:00', '10:00 10:00', '10:00 11:00 12:00',
    '1000 1000', '1000 1000 1000', 't1000', 'T10', 't10:30', '103000', '103000 CET', '1 2 3', '2020-01-01 2020-01-01',
    '99999999999999999999999 days', '+9223372036854775807 sec +1 sec', '+9223372036854775807 sec 1 sec ago',
    '-9223372036854775808 sec -1 sec', '+9223372036854775807 days +1 days', '+999999999999 weeks +999999999999 weeks',
    '+9999999999999 years', '9999999999999 ms', '+9999999999999-01-01', '+99999999999999999999-01-01',
    '-12345-01-01', 'now ago', '2 days ago ago', 'saturday this week', 'weekday', '+2 weekdays', 'weekdays ago',
    'eighth friday of next month', 'twelfth day', 'this', 'next', 'last', "Jan\xC2\xA05", "10:00\xE2\x80\xAFpm",
    "Jan\xC2\xA05\xC2\xA010:00", "\xC2\xA0", "\xFF", "é", "+1\xC2\xA0day", "+1\xE2\x80\xAFday", "next\xC2\xA0week",
    "next\xC2\xA0monday\xC2\xA0of", "first\xE2\x80\xAFmonday\xE2\x80\xAFof", 'next weekday', 'last weekday 10:00',
    '+1 monday 10:00', '10:00 +1 monday 11:00', '10:00 next monday 11:00', 'next week 10:00 11:00', 'Mon, 15 Aug 2005 15:52:01 +0000',
    'Monday, 15-Aug-2005 15:52:01 UTC', 'Mon, 15 Aug 05 15:52:01 +0000', '2005-08-15T15:52:01+00:00',
    'Sat Mar 10 17:16:18 MST 2001', '17:16:18 Mar 10 2001', '03.10.01', '10-3-2001', '2001-03-10 17:16:18',
    '3/10/2001', '3/10', '3/10/01 5pm', '10.3.2001', '10.3.01', '10-3-01', '5-3-2001', '31.12.2020 23:59',
];

$tokens = [
    'now', 'today', 'noon', 'midnight', 'tomorrow', 'yesterday', '@12', '@1.5', 'first day of', 'last day of',
    'back of 7', 'front of 9pm', 'next monday of', 'last sat of', '10am', '10:30pm', '10:30:45 am', '10:30:45:12am',
    't10', '10:30', '10:30:45', '10:30:45.5', '1030', '103045', '1/2', '1/2/2003', '2003/01/02', '2003-01-02',
    '+2003-01-02', '03-01-02', '+12345-01-02', '2003-01', '3-1-2', '2 Jan 2003', '2.1.2003', '2.1.03', 'Jan 2003',
    '2003 Jan', 'Jan 2, 2003', 'Jan 2', '2 Jan', '20030102', '20030102T10:30:45', '2003-01-02T10:30:45.5Z',
    '2003-1-2T10:30:45', '2003:01:02 10:30:45', '2003.002', '2003W01', '2003-W01-2', 'Jan-02-2003', '2003-Jan-02',
    '02/Jan/2003:10:30:45 +0100', '2003', 'ago', 'monday', 'weekday', 'next week', 'this week', 'next month',
    'third day', 'January', 'jan', 'CET', 'Europe/Amsterdam', 'Foo/Bar', 'foo', '+01:00', 'GMT-5', '(UTC)',
    'Jan 2 10:30', 'Jan 2 10.30.45', 'Jan 2 10:30pm', '+1 day', '-2 weeks', '3 hours', '+1 sec', '-1 usec', 'xyz',
    '.', ',', '!', '1', '12', '123', '12345', '1234567', 'II', 'Z', 'p', 'T', 'W', '-', '+', '/', ':', '(', ')',
];
$separators = [' ', '', ',', '  ', 'T', '/', '-', '.', ':', "\t"];

mt_srand(20221017);
$cases = array_fill_keys($fixed, true);
for ($i = 0; $i < $randomCount; $i++) {
    $cases[$tokens[mt_rand(0, count($tokens) - 1)].' '.$tokens[mt_rand(0, count($tokens) - 1)]] = true;
}
for ($i = 0; $i < $randomCount; $i++) {
    $parts = [];
    for ($j = mt_rand(2, 4); $j > 0; $j--) {
        $parts[] = $tokens[mt_rand(0, count($tokens) - 1)];
    }
    $s = '';
    foreach ($parts as $k => $p) {
        $s .= ($k > 0 ? $separators[mt_rand(0, count($separators) - 1)] : '').$p;
    }
    $cases[$s] = true;
}
$alphabet = str_split('0123456789     ::..--//++@,()TtWwZzAaPpMmJjNnUuEeCcGgSsDdYyOoRrHhIiVvXxBbFfKkLl'."\t\0\xC2\xA0\xB5");
for ($i = 0; $i < $randomCount; $i++) {
    $s = '';
    for ($j = mt_rand(1, 16); $j > 0; $j--) {
        $s .= $alphabet[mt_rand(0, count($alphabet) - 1)];
    }
    $cases[$s] = true;
}

$rows = [];
foreach (array_keys($cases) as $s) {
    $s = (string) $s;
    try {
        new DateTime($s);
    } catch (Throwable) {
    }
    $errors = DateTime::getLastErrors();
    $rows[] = [mb_convert_encoding($s, 'UTF-8', 'ISO-8859-1'), $errors === false ? [] : array_values($errors['errors'])];
}
@mkdir(dirname($out), 0777, true);
file_put_contents($out, json_encode($rows, JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
echo count($rows), " cases\n";

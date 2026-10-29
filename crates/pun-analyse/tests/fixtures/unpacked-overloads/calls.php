<?php declare(strict_types=1);

namespace UnpackedOverloads;

function overloads(\PDOStatement $statement, bool $flag): void {
    $statement->fetchAll(...[\PDO::FETCH_COLUMN, 0]);
    $statement->fetchAll(...[\PDO::FETCH_COLUMN, 0, []]);
    $statement->fetchAll(...[\PDO::FETCH_CLASS, \stdClass::class, []]);
    $statement->fetchAll(...[\PDO::FETCH_FUNC, 'intval']);
    $method = 'fetchAll';
    $statement->$method(...[\PDO::FETCH_COLUMN, 0]);
    $statement->$method(...[\PDO::FETCH_COLUMN, 0, []]);
    $statement->$method(...[\PDO::FETCH_CLASS, \stdClass::class, []]);
    $statement->$method(...[\PDO::FETCH_FUNC, 'intval']);

    $arguments = [\PDO::FETCH_COLUMN, 0];
    $statement->fetchAll(...$arguments);
    $statement->$method(...$arguments);
    $arguments = [\PDO::FETCH_COLUMN, 0, []];
    $statement->fetchAll(...$arguments);
    $statement->$method(...$arguments);
    $arguments = [\PDO::FETCH_CLASS, \stdClass::class, []];
    $statement->fetchAll(...$arguments);
    $statement->$method(...$arguments);

    if ($flag) {
        $method = 'fetchAll';
        $arguments = [\PDO::FETCH_COLUMN, 0, []];
    } else {
        $method = 'bindValue';
        $arguments = [1, 'ok'];
    }
    $statement->$method(...$arguments);
}

/** @param array<int, int> $integers
 * @param array<int, string> $strings
 * @param array{int, int}|array{int, string} $alternatives
 * @param array{0: int, 1?: int} $optional
 */
function boundaries(\PDOStatement $statement, array $integers, array $strings, array $alternatives, array $optional): void {
    $method = 'fetchAll';
    $statement->fetchAll(...[]);
    $statement->$method(...[]);
    $statement->fetchAll(\PDO::FETCH_CLASS, ...$strings);
    $statement->$method(\PDO::FETCH_COLUMN, ...$integers);
    $statement->fetchAll(...$alternatives);
    $statement->$method(...$alternatives);
    $statement->fetchAll(...$optional);
    $statement->$method(...$optional);
    $statement->fetchAll(...[\PDO::FETCH_CLASS, \stdClass::class], ...[[]]);
    $statement->$method(...[\PDO::FETCH_CLASS, \stdClass::class], ...[[]]);
    $statement->fetchAll(...[9 => \PDO::FETCH_COLUMN, 2 => 0, 1 => []]);
    $statement->$method(...[9 => \PDO::FETCH_COLUMN, 2 => 0, 1 => []]);
    $statement->fetchAll(...['mode' => \PDO::FETCH_COLUMN, 'args' => 0]);
    $statement->$method(...['mode' => \PDO::FETCH_COLUMN, 'args' => 0]);
    $named = ['mode' => \PDO::FETCH_COLUMN, 'args' => 0];
    $statement->fetchAll(...$named);
    $statement->$method(...$named);
    $statement->fetchAll(...[\PDO::FETCH_COLUMN], mode: \PDO::FETCH_CLASS);
    $statement->$method(...[\PDO::FETCH_COLUMN], mode: \PDO::FETCH_CLASS);
    $statement->fetchAll(...[false, false]);
    $statement->$method(...[false, false]);
}

/** @template T of int
 * @param array{int, T} $arguments
 */
function template(\PDOStatement $statement, array $arguments): void {
    $method = 'fetchAll';
    $statement->fetchAll(...$arguments);
    $statement->$method(...$arguments);
}

/** @param array<int, int> $integers */
function overwrite(\PDOStatement $statement, array $integers): void {
    // The second written argument overwrites selector key 1, rather than unioning string with int.
    // Its int value selects the integer overload, which checks the earlier literal string argument.
    $statement->fetchAll(...[\PDO::FETCH_COLUMN, 'bad'], ...$integers);
    $method = 'fetchAll';
    $statement->$method(...[\PDO::FETCH_COLUMN, 'bad'], ...$integers);
}

function namedConstructors(): void {
    new \DatePeriod(
        start: new \DateTime('now'),
        interval: new \DateInterval('P1D'),
        recurrences: 5,
    );
    new \DatePeriod(isostr: 'R4/2012-07-01T00:00:00Z/P7D');
}

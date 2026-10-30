<?php

namespace App;

interface HasName {}

interface HasAge {}

final class Person implements HasName, HasAge {}

function run(string $class)
{
    // `new $class` is `object`: Pun leaves an `object` value alone (see keep_unknown), where Rector writes `object`.
    tap(new $class, function (Person $value) {
    });
    // Pun does not compare an intersection hint, even in parentheses; Rector writes `\App\Person`.
    tap(new Person, function ((HasName&HasAge)|null $value) {
    });
}

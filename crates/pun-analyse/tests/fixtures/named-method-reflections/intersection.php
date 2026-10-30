<?php

namespace AuditVariants;

interface HasSet
{
    public function set(int $year, int $month, int $third = 1): bool;
}

class Renamed implements HasSet
{
    public function set(int $renamed, int $month, int $third = 1): bool
    {
        return true;
    }
}

function plain(HasSet $calendar): void
{
    $calendar->set(year: 2026, month: 0);
}

function userFirst(HasSet&\IntlCalendar $calendar): void
{
    $calendar->set(year: 2026, month: 0);
}

function builtinFirst(\IntlCalendar&HasSet $calendar): void
{
    $calendar->set(year: 2026, month: 0);
}

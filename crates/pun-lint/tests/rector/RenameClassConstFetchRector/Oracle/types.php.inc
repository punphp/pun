<?php

namespace Rector\Tests\Renaming\Rector\ClassConstFetch\RenameClassConstFetchRector\Fixture;

use Rector\Tests\Renaming\Rector\ClassConstFetch\RenameClassConstFetchRector\Source\LocalFormEvents as Events;
use Rector\Tests\Renaming\Rector\ClassConstFetch\RenameClassConstFetchRector\Source;

class ChildEvents
{
    public function run(Events $events, ?Events $maybe, \stdClass $other)
    {
        return [
            Events::BIND,
            Source\LocalFormEvents::POST_BIND,
            \Rector\Tests\Renaming\Rector\ClassConstFetch\RenameClassConstFetchRector\Source\LocalFormEvents::BIND,
            $events::BIND,
            $maybe::BIND,
            $other::BIND,
            LocalFormEvents::BIND,
            Events::class,
            Events::OLD_CONSTANT,
        ];
    }
}
-----
<?php

namespace Rector\Tests\Renaming\Rector\ClassConstFetch\RenameClassConstFetchRector\Fixture;

use Rector\Tests\Renaming\Rector\ClassConstFetch\RenameClassConstFetchRector\Source\LocalFormEvents as Events;
use Rector\Tests\Renaming\Rector\ClassConstFetch\RenameClassConstFetchRector\Source;

class ChildEvents
{
    public function run(Events $events, ?Events $maybe, \stdClass $other)
    {
        return [
            Events::SUBMIT,
            Source\LocalFormEvents::POST_SUBMIT,
            \Rector\Tests\Renaming\Rector\ClassConstFetch\RenameClassConstFetchRector\Source\LocalFormEvents::SUBMIT,
            $events::SUBMIT,
            $maybe::SUBMIT,
            $other::BIND,
            LocalFormEvents::BIND,
            Events::class,
            \Rector\Tests\Renaming\Rector\ClassConstFetch\RenameClassConstFetchRector\Source\DifferentClass::NEW_CONSTANT,
        ];
    }
}

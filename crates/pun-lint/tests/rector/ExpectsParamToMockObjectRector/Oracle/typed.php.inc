<?php

namespace App\Tests\ExpectsParam;

use PHPUnit\Framework\TestCase;

trait HelperTrait
{
    abstract private function abstractHelper($mock): void;
}

final class ExpectsParamTest extends TestCase
{
    use HelperTrait;

    private function abstractHelper($mock): void
    {
    }

    private function typed(int $count, \stdClass $object, ?array $list, self $self): void
    {
        $count->expects($this->once());
        $object->expects($this->once());
        $list->expects($this->once());
        $self->expects($this->once());
    }
}
-----
<?php

namespace App\Tests\ExpectsParam;

use PHPUnit\Framework\TestCase;

trait HelperTrait
{
    abstract private function abstractHelper($mock): void;
}

final class ExpectsParamTest extends TestCase
{
    use HelperTrait;

    private function abstractHelper($mock): void
    {
    }

    private function typed(\PHPUnit\Framework\MockObject\MockObject $count, \PHPUnit\Framework\MockObject\MockObject $object, \PHPUnit\Framework\MockObject\MockObject $list, \PHPUnit\Framework\MockObject\MockObject $self): void
    {
        $count->expects($this->once());
        $object->expects($this->once());
        $list->expects($this->once());
        $self->expects($this->once());
    }
}

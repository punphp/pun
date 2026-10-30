<?php

namespace QualityPropertyGetterUnionBoundaries;

class BaseType {}

final class GetterUnionBoundaries extends BaseType
{
    private $nullableFalse;
    private $objectAndClass;
    private $objectAndParent;
    private $callable;

    public function getNullableFalse(): false|null
    {
        return $this->nullableFalse;
    }

    public function setNullableFalse(false|null $value): void
    {
        $this->nullableFalse = $value;
    }

    public function getObjectAndClass(): object
    {
        return $this->objectAndClass;
    }

    public function setObjectAndClass(BaseType $value): void
    {
        $this->objectAndClass = $value;
    }

    public function getObjectAndParent(): object
    {
        return $this->objectAndParent;
    }

    public function setObjectAndParent(parent $value): void
    {
        $this->objectAndParent = $value;
    }

    public function getCallable(): callable|string
    {
        return $this->callable;
    }

    public function setCallable(callable|string $value): void
    {
        $this->callable = $value;
    }
}
-----
<?php

namespace QualityPropertyGetterUnionBoundaries;

class BaseType {}

final class GetterUnionBoundaries extends BaseType
{
    private ?false $nullableFalse = null;
    private $objectAndClass;
    private $objectAndParent;
    private $callable;

    public function getNullableFalse(): false|null
    {
        return $this->nullableFalse;
    }

    public function setNullableFalse(false|null $value): void
    {
        $this->nullableFalse = $value;
    }

    public function getObjectAndClass(): object
    {
        return $this->objectAndClass;
    }

    public function setObjectAndClass(BaseType $value): void
    {
        $this->objectAndClass = $value;
    }

    public function getObjectAndParent(): object
    {
        return $this->objectAndParent;
    }

    public function setObjectAndParent(parent $value): void
    {
        $this->objectAndParent = $value;
    }

    public function getCallable(): callable|string
    {
        return $this->callable;
    }

    public function setCallable(callable|string $value): void
    {
        $this->callable = $value;
    }
}

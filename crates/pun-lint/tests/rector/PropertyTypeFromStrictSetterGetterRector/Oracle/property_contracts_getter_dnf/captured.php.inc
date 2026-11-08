<?php

namespace QualityPropertyContracts;

interface GetterLeft {}
interface GetterRight {}
class GetterParent {}

final class GetterDnf extends GetterParent
{
    private $nullableIntersection;
    private $parent;
    private $falseOrClass;
    private $trueOrClass;
    private $onlyNull;

    public function getIntersection(): (GetterLeft&GetterRight)|null
    {
        return $this->nullableIntersection;
    }

    public function setIntersection((GetterLeft&GetterRight)|null $value): void
    {
        $this->nullableIntersection = $value;
    }

    public function getParent(): parent
    {
        return $this->parent;
    }

    public function setParent(parent $value): void
    {
        $this->parent = $value;
    }

    public function getFalseOrClass(): false|GetterParent
    {
        return $this->falseOrClass;
    }

    public function setFalseOrClass(false|GetterParent $value): void
    {
        $this->falseOrClass = $value;
    }

    public function getTrueOrClass(): true|GetterParent
    {
        return $this->trueOrClass;
    }

    public function setTrueOrClass(true|GetterParent $value): void
    {
        $this->trueOrClass = $value;
    }

    public function getNull(): null
    {
        return $this->onlyNull;
    }

    public function setNull(null $value): void
    {
        $this->onlyNull = $value;
    }
}
-----
<?php

namespace QualityPropertyContracts;

interface GetterLeft {}
interface GetterRight {}
class GetterParent {}

final class GetterDnf extends GetterParent
{
    private (\QualityPropertyContracts\GetterLeft&\QualityPropertyContracts\GetterRight)|null $nullableIntersection = null;
    private parent $parent;
    private bool|\QualityPropertyContracts\GetterParent $falseOrClass;
    private bool|\QualityPropertyContracts\GetterParent $trueOrClass;
    private $onlyNull;

    public function getIntersection(): (GetterLeft&GetterRight)|null
    {
        return $this->nullableIntersection;
    }

    public function setIntersection((GetterLeft&GetterRight)|null $value): void
    {
        $this->nullableIntersection = $value;
    }

    public function getParent(): parent
    {
        return $this->parent;
    }

    public function setParent(parent $value): void
    {
        $this->parent = $value;
    }

    public function getFalseOrClass(): false|GetterParent
    {
        return $this->falseOrClass;
    }

    public function setFalseOrClass(false|GetterParent $value): void
    {
        $this->falseOrClass = $value;
    }

    public function getTrueOrClass(): true|GetterParent
    {
        return $this->trueOrClass;
    }

    public function setTrueOrClass(true|GetterParent $value): void
    {
        $this->trueOrClass = $value;
    }

    public function getNull(): null
    {
        return $this->onlyNull;
    }

    public function setNull(null $value): void
    {
        $this->onlyNull = $value;
    }
}

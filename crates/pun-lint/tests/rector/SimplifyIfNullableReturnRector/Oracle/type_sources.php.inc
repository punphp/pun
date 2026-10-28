<?php

namespace App;

final class TypeSources
{
    private function nullFirst(): null|\stdClass
    {
        return null;
    }

    /**
     * @return \ArrayObject<int, int>|null
     */
    private function generic(): ?\ArrayObject
    {
        return null;
    }

    public function docOnMixed()
    {
        /** @var \stdClass|null $value */
        $value = $this->foo->bar();
        if (! $value instanceof \stdClass) {
            return null;
        }

        return $value;
    }

    public function nativeNullFirst()
    {
        $value = $this->nullFirst();
        if (! $value instanceof \stdClass) {
            return null;
        }

        return $value;
    }

    public function genericObject()
    {
        $value = $this->generic();
        if (! $value instanceof \ArrayObject) {
            return null;
        }

        return $value;
    }

    public function shortName()
    {
        $value = $this->generic();
        if (! $value instanceof ArrayObject) {
            return null;
        }

        return $value;
    }
}
-----
<?php

namespace App;

final class TypeSources
{
    private function nullFirst(): null|\stdClass
    {
        return null;
    }

    /**
     * @return \ArrayObject<int, int>|null
     */
    private function generic(): ?\ArrayObject
    {
        return null;
    }

    public function docOnMixed()
    {
        /** @var \stdClass|null $value */
        $value = $this->foo->bar();
        if (! $value instanceof \stdClass) {
            return null;
        }

        return $value;
    }

    public function nativeNullFirst()
    {
        return $this->nullFirst();
    }

    public function genericObject()
    {
        return $this->generic();
    }

    public function shortName()
    {
        $value = $this->generic();
        if (! $value instanceof ArrayObject) {
            return null;
        }

        return $value;
    }
}

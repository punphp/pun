<?php

namespace Probe;

final class Connection
{
    /**
     * @var bool
     */
    protected $modified = false;

    protected $other;

    protected $third;

    protected bool $typed = false;

    public function falsy($value = true)
    {
        if (! $this->modified) {
            $this->modified = $value;
        }
    }

    public function plain($value)
    {
        $this->other = $value;
    }

    public function nullCheck($value)
    {
        if ($this->third === null) {
            $this->third = $value;
        }
    }

    public function truthy($value)
    {
        if ($this->other) {
            $this->other = $value;
        }
    }

    public function typedFalsy($value)
    {
        if (! $this->typed) {
            $this->typed = $value;
        }
    }
}
-----
<?php

namespace Probe;

final class Connection
{
    /**
     * @var bool
     */
    protected $modified = false;

    protected $other;

    protected $third;

    protected bool $typed = false;

    public function falsy(array|bool|int|float|string|null $value = true)
    {
        if (! $this->modified) {
            $this->modified = $value;
        }
    }

    public function plain($value)
    {
        $this->other = $value;
    }

    public function nullCheck(null $value)
    {
        if ($this->third === null) {
            $this->third = $value;
        }
    }

    public function truthy($value)
    {
        if ($this->other) {
            $this->other = $value;
        }
    }

    public function typedFalsy(bool $value)
    {
        if (! $this->typed) {
            $this->typed = $value;
        }
    }
}

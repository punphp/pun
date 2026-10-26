<?php

class Base
{
}

class Upload extends Base
{
    public function __construct($path = '')
    {
    }

    /**
     * @return static
     */
    public static function createFromBase(Base $file)
    {
        return $file instanceof static ? $file : new static('x');
    }
}

class Once
{
    protected static ?self $instance = null;

    protected function __construct()
    {
    }

    /**
     * @return static
     */
    public static function instance()
    {
        return static::$instance ??= new static();
    }
}

trait WithContainer
{
    protected function mockVite()
    {
        return new class {
            public function useHotFile($path)
            {
                return $this;
            }
        };
    }
}
-----
<?php

class Base
{
}

class Upload extends Base
{
    public function __construct($path = '')
    {
    }

    /**
     * @return static
     */
    public static function createFromBase(Base $file): static
    {
        return $file instanceof static ? $file : new static('x');
    }
}

class Once
{
    protected static ?self $instance = null;

    protected function __construct()
    {
    }

    /**
     * @return static
     */
    public static function instance(): self
    {
        return static::$instance ??= new static();
    }
}

trait WithContainer
{
    protected function mockVite()
    {
        return new class {
            public function useHotFile($path): self
            {
                return $this;
            }
        };
    }
}

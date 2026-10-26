<?php

final class AnonymousFluent
{
    public function make()
    {
        return new class {
            public $afterCommit = true;

            public function beforeCommit()
            {
                $this->afterCommit = false;

                return $this;
            }
        };
    }

    public function variable()
    {
        $job = new class {
        };

        return $job;
    }
}
-----
<?php

final class AnonymousFluent
{
    public function make(): object
    {
        return new class {
            public $afterCommit = true;

            public function beforeCommit()
            {
                $this->afterCommit = false;

                return $this;
            }
        };
    }

    public function variable(): object
    {
        $job = new class {
        };

        return $job;
    }
}

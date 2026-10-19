<?php

try {
    a();
} catch (PDOException $e) {
    echo "failed: $e {$e->getMessage()}";
    $f = function ($e) use ($e) {
        return fn () => $e;
    };
}
echo $e;
b($e);
c($e);
d($e);
e($e);
if ($e) {
    $x = 1;
    echo $e;
}
try {
} catch (AbstractFactoryInterface $e) {
}
try {
} catch (Foo\Bar\IFactory $e) {
}
try {
} catch (\Foo\Bar\FactoryInterface $e) {
}
try {
} catch (My_Odd_Exception $e) {
}
try {
} catch (IOException $e) {
}
try {
} catch (A|B $e) {
}
try {
} catch (A $e) {
} catch (B $e) {
}
try {
} catch (Exception) {
}
if (true) {
    try {
    } catch (Exception $e) {
    }
}

function f()
{
    try {
    } catch (Exception $ex) {
    }
    g($ex);
    try {
    } catch (RuntimeException $ex) {
    }
    g($ex);
    try {
    } catch (Exception $ex) {
    }
}

function g(Exception $exception)
{
    try {
    } catch (LogicException $err) {
    }
    try {
    } catch (Exception $err) {
    }
    try {
    } catch (Error $err) {
    }
}

class K
{
    public function m()
    {
        try {
        } catch (Exception $a) {
        }
        $a->x = 1;
        $b = $a;
        h($a);
        try {
        } catch (Throwable $a) {
            $a = 1;
        }
        i($a);
        try {
        } catch (Exception) {
        }
        j($a);
    }
}
-----
<?php

try {
    a();
} catch (PDOException $pdoException) {
    echo "failed: $pdoException {$pdoException->getMessage()}";
    $f = function ($pdoException) use ($pdoException) {
        return fn () => $pdoException;
    };
}
echo $pdoException;
b($pdoException);
c($e);
d($pdoException);
e($e);
if ($pdoException) {
    $x = 1;
    echo $e;
}
try {
} catch (AbstractFactoryInterface $Factory) {
}
try {
} catch (Foo\Bar\IFactory $factory) {
}
try {
} catch (\Foo\Bar\FactoryInterface $barFactory) {
}
try {
} catch (My_Odd_Exception $myOddException) {
}
try {
} catch (IOException $ioException) {
}
try {
} catch (A|B $e) {
}
try {
} catch (A $e) {
} catch (B $e) {
}
try {
} catch (Exception) {
}
if (true) {
    try {
    } catch (Exception $e) {
    }
}

function f()
{
    try {
    } catch (Exception $exception) {
    }
    g($exception);
    try {
    } catch (RuntimeException $runtimeException) {
    }
    g($runtimeException);
    try {
    } catch (Exception $exception) {
    }
}

function g(Exception $exception)
{
    try {
    } catch (LogicException $logicException) {
    }
    try {
    } catch (Exception $err) {
    }
    try {
    } catch (Error $err) {
    }
}

class K
{
    public function m()
    {
        try {
        } catch (Exception $exception) {
        }
        $exception->x = 1;
        $b = $a;
        h($a);
        try {
        } catch (Throwable $throwable) {
            $throwable = 1;
        }
        i($throwable);
        try {
        } catch (Exception) {
        }
        j($a);
    }
}

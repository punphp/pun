<?php

namespace App;

class Failure extends \RuntimeException
{
}

function run()
{
    try {
        work();
    } catch (\LogicException | \TypeError $e) {
        throw $e;
    } catch (\Throwable $e) {
        report($e);
    }

    try {
        work();
    } catch (Failure $e) {
        throw $e;
    } catch (\LogicException | \RuntimeException $e) {
        report($e);
    }

    try {
        work();
    } catch (Failure $e) {
        throw $e;
    } catch (\RuntimeException $e) {
        report($e);
    }

    try {
        work();
    } catch (Failure $e) {
        throw $e;
    } catch (\RuntimeException $e) {
        throw $e;
    } catch (\Exception $e) {
        report($e);
    }

    try {
        work();
    } catch (Failure) {
        throw new \Exception();
    } catch (\Exception $e) {
        throw $e;
    }

    try {
        work();
    } catch (Failure $e) {
        report($e);
        throw $e;
    } catch (\Exception $e) {
        throw $e;
    }

    try {
        work();
    } catch (\Exception $e) {
        throw $e;
    } catch (\Error $e) {
        throw $e;
    }

    try {
        work();
    } catch (\Exception $e) {
        throw ($e);
    } catch (\Error $e) {
    }
}
-----
<?php

namespace App;

class Failure extends \RuntimeException
{
}

function run()
{
    try {
        work();
    } catch (\LogicException | \TypeError $e) {
        throw $e;
    } catch (\Throwable $e) {
        report($e);
    }

    try {
        work();
    } catch (Failure $e) {
        throw $e;
    } catch (\LogicException | \RuntimeException $e) {
        report($e);
    }

    try {
        work();
    } catch (Failure $e) {
        throw $e;
    } catch (\RuntimeException $e) {
        report($e);
    }

    try {
        work();
    } catch (Failure $e) {
        throw $e;
    } catch (\RuntimeException $e) {
        throw $e;
    } catch (\Exception $e) {
        report($e);
    }

    try {
        work();
    } catch (Failure) {
        throw new \Exception();
    }

    try {
        work();
    } catch (Failure $e) {
        report($e);
        throw $e;
    }

    try {
        work();
    } catch (\Exception $e) {
        throw $e;
    } catch (\Error $e) {
        throw $e;
    }

    try {
        work();
    } catch (\Error $e) {
    }
}

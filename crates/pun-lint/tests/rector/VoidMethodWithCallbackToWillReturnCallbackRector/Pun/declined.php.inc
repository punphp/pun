<?php

namespace Pun\Vm;

use PHPUnit\Framework\TestCase;

final class DeclinedTest extends TestCase
{
    public function testDeclined(): void
    {
        $m = $this->createMock(Manager::class);
        $m->method('persist')->with(/* note */ $this->callback(function ($e) {
            echo 1;
            return true;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            echo 1;
            ?><?php
            return true;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            if ($e) {
                return (true);
            }
            echo 1;
        }));
        $m->method('persist')->with($this->callback(
function ($e) {
    echo 1;
    return true;
}));
        $m->method('persist')->with($this->callback(function (/* any */ $e) {
            return true;
        }));
        $m->method('persist')->with($this->callback(function (
            $e,
            $f = [
                1,
            ],
        ) {
            return true;
        }));
	$m->method('persist')->with($this->callback(function ($e) {
		return true;
	}));
	$m->method('persist')->with(
		$this->callback(
			function ($e) {
				echo 1;
				return true;
			}
		)
	);
    }
}

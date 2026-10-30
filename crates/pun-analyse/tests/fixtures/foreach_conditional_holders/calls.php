<?php declare(strict_types=1);

namespace ForeachConditionalHolders;

class Target {
    public function fetchTest(int $i): void {}
    public function fetchTest2(): void {}

    /** @param array<'test1'|'test2', 'Test'|'Test2'> $list */
    public function generic(array $list): void {
        foreach ($list as $key => $functionName) {
            $functionToCall = 'fetch' . $functionName;
            if ($key === 'test1') {
                $this->$functionToCall(1);
            } else {
                $this->$functionToCall();
            }
        }
    }

    public function overwritten(): void {
        $list = ['test1' => 'Test', 'test2' => 'Test2'];
        foreach ($list as $key => $functionName) {
            $functionToCall = 'fetch' . $functionName;
            $functionToCall = 'fetchTest';
            if ($key === 'test2') {
                $this->$functionToCall();
            }
        }
    }

    public function reference(): void {
        $list = ['test1' => 'Test', 'test2' => 'Test2'];
        foreach ($list as $key => &$functionName) {
            $functionToCall = 'fetch' . $functionName;
            if ($key === 'test1') {
                $this->$functionToCall(1);
            } else {
                $this->$functionToCall();
            }
        }
    }
}

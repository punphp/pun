<?php

namespace Probe;

final class T
{
    public function assertSame($a, $b): void
    {
    }

    public function run()
    {
        $assert = function ($users) {
            $assertions = [
                ['first_name' => 'Thomas', 'last_name' => 'Anderson'],
                ['first_name' => 'Thomas', 'last_name' => 'Smith'],
            ];

            foreach ($assertions as $key => $assertion) {
                $this->assertSame(
                    $assertion,
                    $users[$key]->only('first_name', 'last_name'),
                );
            }
        };

        $assert([]);
    }
}

<?php

namespace Laravel\Prompts\Output;

use Symfony\Component\Console\Output\ConsoleOutput as SymfonyConsoleOutput;

class ConsoleOutput extends SymfonyConsoleOutput
{
    /**
     * How many new lines were written by the last output.
     */
    protected int $newLinesWritten = 1;

    /**
     * How many new lines were written by the last output.
     */
    public function newLinesWritten(): int
    {}

    /**
     * Write the output and capture the number of trailing new lines.
     */
    protected function doWrite(string $message, bool $newline): void
    {}

    /**
     * Write output directly, bypassing newline capture.
     */
    public function writeDirectly(string $message): void
    {}
}

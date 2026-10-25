<?php

namespace TraitMethodOwners;

/** @method void documented() */
trait Output
{
    public function line(): void {}

    protected function protectedLine(): void {}

    private function privateLine(): void {}
}

class Command
{
    use Output { line as repeated; }

    public function own(): void
    {
        $value = $this->line();
        $value = $this->documented();
        $this->protectedLine();
        $this->privateLine();
        $this->repeated();
    }
}

class ChildCommand extends Command
{
    public function inherited(): void
    {
        $value = $this->line();
        $value = $this->documented();
        $this->protectedLine();
        $this->privateLine();
        $this->repeated();
    }
}

class ReusingCommand extends Command
{
    use Output;

    public function reused(): void
    {
        $value = $this->line();
        $value = $this->documented();
        $this->privateLine();
    }
}

trait WrappedOutput
{
    use Output { line as nestedAlias; }
}

class NestedCommand
{
    use WrappedOutput { line as outerAlias; }

    public function nested(): void
    {
        $value = $this->line();
        $value = $this->documented();
        $this->protectedLine();
        $this->privateLine();
        $this->nestedAlias();
        $this->outerAlias();
    }
}

class NestedChildCommand extends NestedCommand
{
    public function inheritedNested(): void
    {
        $value = $this->line();
        $value = $this->documented();
        $this->protectedLine();
        $this->nestedAlias();
        $this->outerAlias();
    }
}

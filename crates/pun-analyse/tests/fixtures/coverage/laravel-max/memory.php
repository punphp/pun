<?php

namespace LaravelMax\Memory;

class Command
{
    /** @return string|array<string>|bool|null */
    public function option(string $key)
    {
        return null;
    }

    public function takesString(string $string): void
    {
    }

    public function rememberedCalls(): void
    {
        if ($this->option('file')) {
            $this->takesString($this->option('file'));
        }

        if (is_string($this->option('name'))) {
            $this->takesString($this->option('name'));
            $this->takesString($this->option('other'));
        }
    }
}

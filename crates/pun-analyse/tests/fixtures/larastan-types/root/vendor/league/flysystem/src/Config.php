<?php

declare(strict_types=1);

namespace League\Flysystem;

use function array_diff_key;
use function array_flip;
use function array_merge;

class Config
{
    public const OPTION_COPY_IDENTICAL_PATH = 'copy_destination_same_as_source';
    public const OPTION_MOVE_IDENTICAL_PATH = 'move_destination_same_as_source';
    public const OPTION_VISIBILITY = 'visibility';
    public const OPTION_DIRECTORY_VISIBILITY = 'directory_visibility';
    public const OPTION_RETAIN_VISIBILITY = 'retain_visibility';

    public function __construct(private array $options = [])
    {}

    /**
     * @param mixed $default
     *
     * @return mixed
     */
    public function get(string $property, $default = null)
    {}

    public function extend(array $options): Config
    {}

    public function withDefaults(array $defaults): Config
    {}

    public function toArray(): array
    {}

    public function withSetting(string $property, mixed $setting): Config
    {}

    public function withoutSettings(string ...$settings): Config
    {}
}

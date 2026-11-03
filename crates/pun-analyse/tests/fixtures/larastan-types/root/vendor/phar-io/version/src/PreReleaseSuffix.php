<?php declare(strict_types = 1);
namespace PharIo\Version;

class PreReleaseSuffix {
    private const valueScoreMap = [
        'dev'   => 0,
        'a'     => 1,
        'alpha' => 1,
        'b'     => 2,
        'beta'  => 2,
        'rc'    => 3,
        'p'     => 4,
        'pl'    => 4,
        'patch' => 4,
    ];

    /** @var string */
    private $value;

    /** @var int */
    private $valueScore;

    /** @var int */
    private $number = 0;

    /** @var string */
    private $full;

    /**
     * @throws InvalidPreReleaseSuffixException
     */
    public function __construct(string $value) {}

    public function asString(): string {}

    public function getValue(): string {}

    public function getNumber(): ?int {}

    public function isGreaterThan(PreReleaseSuffix $suffix): bool {}

    private function mapValueToScore(string $value): int {}

    private function parseValue(string $value): void {}
}

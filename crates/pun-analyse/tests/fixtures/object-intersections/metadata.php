<?php
namespace ExactValueMetadata;

class Service {
    public const VALUE = 1;
    public function read(): void {
        echo $this::$missing;
    }
}

/** @template T */
class Generic {
    public function read(): void {
        echo $this->missing;
    }
}

function constant_receiver(bool $flag): void {
    $receiver = $flag ? Service::class : new Service();
    echo $receiver::VALUE;
}

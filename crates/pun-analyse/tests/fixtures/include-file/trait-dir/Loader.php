<?php
namespace IncludeFileTraits;
trait Loader {
    public function load(): void {
        include 'trait.txt';
        include 'class.txt';
        include __DIR__ . '/trait.txt';
        include __DIR__ . '/class.txt';
        include __FILE__;
    }
}

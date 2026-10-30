<?php

namespace App;

abstract class A
{
    public function a()
    {
        tap(new static, function (A $i) {
        });
        tap(new self, function ($i) {
        });
        tap(new self(), function (self $i) {
        });
        tap(new B, function ($i) {
        });
        tap(new b, function ($i) {
        });
        tap(new b, function (B $i) {
        });
        tap(new \App\B(1), function (b $i) {
        });
        tap((new B), function ($i) {
        });
    }
}

class B extends A
{
    public function c()
    {
        tap(new parent, function ($i) {
        });
    }
}
-----
<?php

namespace App;

abstract class A
{
    public function a()
    {
        tap(new static, function ($i) {
        });
        tap(new self, function (\App\A $i) {
        });
        tap(new self(), function (self $i) {
        });
        tap(new B, function (\App\B $i) {
        });
        tap(new b, function (\App\b $i) {
        });
        tap(new b, function (\App\b $i) {
        });
        tap(new \App\B(1), function (\App\B $i) {
        });
        tap((new B), function (\App\B $i) {
        });
    }
}

class B extends A
{
    public function c()
    {
        tap(new parent, function (\App\A $i) {
        });
    }
}

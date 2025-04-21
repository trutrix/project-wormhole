use crate::dev::*;

#[derive(Debug, NomLE)]
pub struct Vec2<T> (pub [T;2]);

#[derive(Debug, NomLE)]
pub struct Vec3<T> (pub [T;3]);

#[derive(Debug, NomLE)]
pub struct Vec4<T> (pub [T;4]);
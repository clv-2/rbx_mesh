use std::io::Read;
use binrw::BinWrite;

use crate::mesh::*;
fn load_mesh(name:&str)->Result<Mesh,Error>{
	read_versioned(std::fs::File::open(name).map_err(Error::Io)?)
}
fn get_mesh_id(mesh:Mesh)->u16{
	match mesh{
		Mesh::V1(mesh)=>match mesh.header.revision{
			Revision1::Version100=>100,
			Revision1::Version101=>101,
		},
		Mesh::V2(mesh)=>match mesh.header.revision{
			Revision2::Version200=>200,
		},
		Mesh::V3(mesh)=>match mesh.header.revision{
			Revision3::Version300=>300,
			Revision3::Version301=>301,
		},
		Mesh::V4(mesh)=>match mesh.header.revision{
			Revision4::Version400=>400,
			Revision4::Version401=>401,
		},
		Mesh::V5(mesh)=>match mesh.header.revision{
			Revision5::Version500=>500,
		},
	}
}
//Mesh1 has no round trip since there is no writer
fn round_trip2(name:&str){
	let mut file=std::fs::File::open(name).unwrap();
	let mut rbuf=Vec::new();
	let mut wbuf=Vec::new();
	file.read_to_end(&mut rbuf).unwrap();
	//read and then write mesh
	read2(binrw::io::Cursor::new(&rbuf)).unwrap()
	.write_le(&mut binrw::io::Cursor::new(&mut wbuf)).unwrap();
	assert_eq!(rbuf,wbuf);
}
fn round_trip3(name:&str){
	let mut file=std::fs::File::open(name).unwrap();
	let mut rbuf=Vec::new();
	let mut wbuf=Vec::new();
	file.read_to_end(&mut rbuf).unwrap();
	read3(binrw::io::Cursor::new(&rbuf)).unwrap()
	.write_le(&mut binrw::io::Cursor::new(&mut wbuf)).unwrap();
	assert_eq!(rbuf,wbuf);
}
fn round_trip4(name:&str){
	let mut file=std::fs::File::open(name).unwrap();
	let mut rbuf=Vec::new();
	let mut wbuf=Vec::new();
	file.read_to_end(&mut rbuf).unwrap();
	read4(binrw::io::Cursor::new(&rbuf)).unwrap()
	.write_le(&mut binrw::io::Cursor::new(&mut wbuf)).unwrap();
	assert_eq!(rbuf,wbuf);
}
fn round_trip5(name:&str){
	let mut file=std::fs::File::open(name).unwrap();
	let mut rbuf=Vec::new();
	let mut wbuf=Vec::new();
	file.read_to_end(&mut rbuf).unwrap();
	read5(binrw::io::Cursor::new(&rbuf)).unwrap()
	.write_le(&mut binrw::io::Cursor::new(&mut wbuf)).unwrap();
	assert_eq!(rbuf,wbuf);
}
#[test]
fn mesh_100(){
	assert_eq!(get_mesh_id(load_mesh("meshes/158071912").unwrap()),100);
}
#[test]
fn mesh_200(){
	assert_eq!(get_mesh_id(load_mesh("meshes/torso.mesh").unwrap()),200)
}
#[test]
fn roundtrip_200(){
	round_trip2("meshes/torso.mesh");
}
#[test]
fn mesh_300(){
	assert_eq!(get_mesh_id(load_mesh("meshes/5115672913").unwrap()),300);
}
#[test]
fn roundtrip_300(){
	round_trip3("meshes/5115672913");
}
#[test]
fn mesh_301(){
	assert_eq!(get_mesh_id(load_mesh("meshes/5648093777").unwrap()),301)
}
#[test]
fn roundtrip_301(){
	round_trip3("meshes/5648093777");
}
#[test]
fn mesh_401(){
	assert_eq!(get_mesh_id(load_mesh("meshes/sphere.mesh").unwrap()),401)
}
#[test]
fn roundtrip_401(){
	round_trip4("meshes/sphere.mesh");
}
#[test]
fn mesh_401_random_padding(){
	assert_eq!(get_mesh_id(load_mesh("meshes/7665777615").unwrap()),401)
}
#[test]
fn roundtrip_401_random_padding(){
	round_trip4("meshes/7665777615");
}
//the only three v5.00 meshes I could find in bhop and surf
#[test]
fn mesh_500(){
	assert_eq!(get_mesh_id(load_mesh("meshes/13674780763").unwrap()),500)
}
#[test]
fn roundtrip_500(){
	round_trip5("meshes/13674780763");
}
#[test]
fn mesh_500_alt1(){
	assert_eq!(get_mesh_id(load_mesh("meshes/14818281896").unwrap()),500)
}
#[test]
fn roundtrip_500_alt1(){
	round_trip5("meshes/14818281896");
}
#[test]
fn mesh_500_alt2(){
	assert_eq!(get_mesh_id(load_mesh("meshes/15256456161").unwrap()),500)
}
#[test]
fn roundtrip_500_alt2(){
	round_trip5("meshes/15256456161");
}
//also tested against ~2500 meshes from bhop and surf maps

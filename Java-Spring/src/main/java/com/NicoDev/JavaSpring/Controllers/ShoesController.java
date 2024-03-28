package com.NicoDev.JavaSpring.Controllers;

import java.util.List;

import org.bson.types.ObjectId;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RestController;

import com.NicoDev.JavaSpring.Models.ShoeModel;
import com.NicoDev.JavaSpring.Services.ShoesServices;

@RestController
public class ShoesController {

    @Autowired
    private ShoesServices shoesServices;

    @GetMapping(value = "/shoes", produces = "application/json")
    public List<ShoeModel> getShoes() {
        return shoesServices.getAllShoes();
    }

    @GetMapping(value = "/shoes/{shoeId}")
public ResponseEntity<ShoeModel> getShoe(@PathVariable ObjectId shoeId) {
    ShoeModel shoe = shoesServices.getShoe(shoeId);
    if (shoe != null) {
        return new ResponseEntity<>(shoe, HttpStatus.OK);
    } else {
        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }
}

    @PostMapping(value = "/shoes")
    public ResponseEntity<Void> createShoe(@RequestBody ShoeModel shoe){
        shoesServices.createShoe(shoe);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

@PutMapping(value = "/shoes/{shoeId}")
public ResponseEntity<Void> updateShoe(@PathVariable ObjectId shoeId, @RequestBody  ShoeModel shoe) {
    shoesServices.updateShoe(shoeId, shoe);
    return new ResponseEntity<>(HttpStatus.OK);
}

    @DeleteMapping(value = "/shoes/{shoeId}")
    public ResponseEntity<Void> deleteShoe(@PathVariable ObjectId shoeId){
shoesServices.deleteShoe(shoeId);
    return new ResponseEntity<>(HttpStatus.NO_CONTENT);
    }

}

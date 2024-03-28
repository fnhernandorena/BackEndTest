package com.NicoDev.JavaSpring.Services;

import java.util.List;

import org.bson.types.ObjectId;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import com.NicoDev.JavaSpring.Repositories.ShoesRepo;
import com.NicoDev.JavaSpring.Models.ShoeModel  ;

@Service
public class ShoesServices {

    @Autowired
    ShoesRepo repository;

    public List<ShoeModel> getAllShoes() {
    return repository.findAll();
    }   

    public ShoeModel getShoe(ObjectId shoeId) {
        if (shoeId == null) {
            return null;
        }
        return repository.findById(shoeId).orElse(null);
    }

    public void createShoe(ShoeModel shoe){
        ShoeModel newShoe = new ShoeModel(
            shoe.getBrand(),
            shoe.getModel(),
            shoe.getSize()
        );

        repository.save(newShoe);
    }

    public void updateShoe(ObjectId shoeId, ShoeModel shoe) {
        if (shoeId!=null) {
            ShoeModel updatedShoe = repository.findById(shoeId).orElse(null);
    
        if (updatedShoe != null && shoe != null) {
            updatedShoe.setBrand(shoe.getBrand());
            updatedShoe.setModel(shoe.getModel());
            updatedShoe.setSize(shoe.getSize());
    
            repository.save(updatedShoe);
        }
        }
        
    }

    public void deleteShoe(ObjectId shoeId) {
        if (shoeId!=null) {
            ShoeModel shoe = repository.findById(shoeId).orElse(null);

        if (shoe != null) {
            repository.delete(shoe);
        }
        }
        
    }

}

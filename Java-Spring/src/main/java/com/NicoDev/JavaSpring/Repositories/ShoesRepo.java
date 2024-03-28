package com.NicoDev.JavaSpring.Repositories;

import org.bson.types.ObjectId;
import org.springframework.data.mongodb.repository.MongoRepository;
import org.springframework.stereotype.Repository;

import com.NicoDev.JavaSpring.Models.ShoeModel;

@Repository
public interface ShoesRepo extends MongoRepository<ShoeModel, ObjectId> {

}

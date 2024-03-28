package com.NicoDev.JavaSpring.Models;

import java.io.Serializable;

import org.springframework.data.mongodb.core.mapping.Document;

import com.mongodb.lang.NonNull;

import jakarta.persistence.Id;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
@Document(collection = "shoes_java")

public class ShoeModel implements Serializable{

    @Id
    @NonNull
    private String id;
    private String brand;
    private String model;
    private Integer size;

    public ShoeModel( String brand, String model, Integer size) {
        this.brand = brand;
        this.model = model;
        this.size = size;
    }

}

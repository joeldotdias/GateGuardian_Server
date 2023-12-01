package com.gateguardian.gateguardianserver.user.model;

import jakarta.persistence.*;

@Entity
@Table(name = "users")
public class User {

   @Id
   @GeneratedValue(strategy = GenerationType.IDENTITY)
   private Integer id;

   @Column(name = "email")
   private String email;

   @Column(name = "category")
   private String category;

   public User() {

   }

   public User(Integer id, String email, String category) {
      this.id = id;
      this.email = email;
      this.category = category;
   }

   public Integer getId() {
      return id;
   }

   public void setId(Integer id) {
      this.id = id;
   }

   public String getEmail() {
      return email;
   }

   public void setEmail(String email) {
      this.email = email;
   }

   public String getCategory() {
      return category;
   }

   public void setCategory(String category) {
      this.category = category;
   }


}
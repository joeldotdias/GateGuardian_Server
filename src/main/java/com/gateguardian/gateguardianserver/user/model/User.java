package com.gateguardian.gateguardianserver.user.model;

import jakarta.persistence.*;

@Entity
@Table(name = "users")
public class User {

   @Id
   @GeneratedValue(strategy = GenerationType.IDENTITY)
   private Integer id;

   @Column(name = "name")
   private String name;

   @Column(name = "email")
   private String email;

   @Column(name = "category")
   private String category;

   @Column(name = "society")
   private String society;

   public User() {}

   public User(String name, String email, String category, String society) {
      this.name = name;
      this.email = email;
      this.category = category;
      this.society = society;
   }

   public User(Integer id, String name, String email, String category, String society) {
      this.id = id;
      this.name = name;
      this.email = email;
      this.category = category;
      this.society = society;
   }

   public Integer getId() {
      return id;
   }
   public void setId(Integer id) {
      this.id = id;
   }

   public String getName() { return name; }
   public void setName(String name) { this.name = name; }

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

   public String getSociety() { return society; }
   public void setSociety(String society) { this.society = society; }
}
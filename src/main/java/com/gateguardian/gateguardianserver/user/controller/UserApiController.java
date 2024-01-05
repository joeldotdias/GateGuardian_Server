package com.gateguardian.gateguardianserver.user.controller;

import com.gateguardian.gateguardianserver.user.model.User;
import com.gateguardian.gateguardianserver.user.service.UserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

@RestController
public class UserApiController {

   @Autowired
   private UserService userService;

   @GetMapping("/user")
   public User getUserByEmail(
           @RequestParam(name = "email") String email
   ) {
      return userService.getUserByEmail(email);
   }

   @PostMapping("/user-save")
   public void saveUser(
           @RequestBody User user
   ) {
      userService.saveUser(user);
   }
}
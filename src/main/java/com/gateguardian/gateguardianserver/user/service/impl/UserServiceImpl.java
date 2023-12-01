package com.gateguardian.gateguardianserver.user.service.impl;

import com.gateguardian.gateguardianserver.user.model.User;
import com.gateguardian.gateguardianserver.user.repository.UserRepository;
import com.gateguardian.gateguardianserver.user.service.UserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class UserServiceImpl implements UserService {

   @Autowired
   private UserRepository userRepository;

   @Override
   public List<User> getUserByEmail(String email) {
      return userRepository.getUserByEmail(email);
   }

   @Override
   public void saveUser(User user) {
      userRepository.save(user);
   }
}
package net.yageek.testbitmap;

import android.graphics.Bitmap;
import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;

import androidx.annotation.NonNull;
import androidx.fragment.app.Fragment;
import androidx.navigation.fragment.NavHostFragment;

import net.yageek.testbitmap.databinding.FragmentFirstBinding;

import java.util.Arrays;

public class FirstFragment extends Fragment {

    private FragmentFirstBinding binding;

    @Override
    public View onCreateView(
            LayoutInflater inflater, ViewGroup container,
            Bundle savedInstanceState
    ) {

        binding = FragmentFirstBinding.inflate(inflater, container, false);
        return binding.getRoot();

    }

    public void onViewCreated(@NonNull View view, Bundle savedInstanceState) {
        super.onViewCreated(view, savedInstanceState);

        int[] colors = new int[10000];
        Arrays.fill(colors, 0, 4999, 0xffffffff);
        Arrays.fill(colors, 5000, 9999, 0xff000000);
       Bitmap bt = Bitmap.createBitmap(colors, 100, 100, Bitmap.Config.ARGB_8888);

       binding.imageView.setImageBitmap(bt);

    }

    @Override
    public void onDestroyView() {
        super.onDestroyView();
        binding = null;
    }

}